"""Tachikoma gRPC server."""
from concurrent import futures
import argparse
import contextlib
import logging
import multiprocessing
import signal
import socket
import sys

import grpc

from db import DB
from tachikoma import Tachikoma
from tachikoma_pb2_grpc import add_TachikomaServicer_to_server
import utils as u

logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s [PID %(process)d]: %(message)s',
    datefmt='%m/%d/%Y %I:%M:%S %p',
)


class ServerLoggingInterceptor(grpc.ServerInterceptor):
    """Intercepts and logs calls."""
    def intercept_service(self, continuation, handler_call_details):
        """Intercepts and logs calls."""
        logging.info(handler_call_details.method)
        return continuation(handler_call_details)


@contextlib.contextmanager
def _reserve_port(port):
    """Reserves a port for all processes."""
    sock = socket.socket(socket.AF_INET6, socket.SOCK_STREAM)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEPORT, 1)
    if sock.getsockopt(socket.SOL_SOCKET, socket.SO_REUSEPORT) == 0:
        raise RuntimeError('Failed to set SO_REUSEPORT')
    sock.bind(('', port))
    try:
        yield sock.getsockname()[1]
    finally:
        sock.close()


def _run_server(address, db_name, n_threads_per_process):
    """Start a server."""
    server = grpc.server(
        futures.ThreadPoolExecutor(max_workers=n_threads_per_process),
        options=(('grpc.so_reuseport', 1), ),
        interceptors=(ServerLoggingInterceptor(), ),
    )
    db = DB(db_name)
    add_TachikomaServicer_to_server(Tachikoma(db), server)
    server.add_insecure_port(address)
    server.start()
    try:
        logging.info('started server')
        signal.pause()
    except KeyboardInterrupt:
        logging.info('stopping server')
        server.stop(None)


def parse_args(argv):
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        prog=argv[0],
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument(
        '-db',
        '--database',
        default='motoko',
        choices=['motoko', 'motoko_test'],
    )
    parser.add_argument(
        '-p',
        '--port',
        type=int,
        default=9001,
    )
    parser.add_argument(
        '-np',
        '--n_processes',
        type=int,
        default=multiprocessing.cpu_count(),
    )
    parser.add_argument(
        '-nt',
        '--n_threads_per_process',
        type=int,
        default=multiprocessing.cpu_count(),
    )
    return parser.parse_args(argv[1:])


if __name__ == '__main__':
    with open('motd.txt') as f:
        print(f.read())
    args = parse_args(sys.argv)
    with _reserve_port(args.port) as port:
        address = f'localhost:{port}'
        logging.info(f'binding to {address}')
        workers = []
        for _ in range(args.n_processes):
            worker = multiprocessing.Process(
                target=_run_server,
                args=(address, args.database, args.n_threads_per_process),
            )
            worker.start()
            workers.append(worker)
        try:
            signal.pause()
        except KeyboardInterrupt:
            for worker in workers:
                worker.join()
