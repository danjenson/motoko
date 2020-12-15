import socket, struct


def lambda_handler(event, context):
    """Read the default gateway directly from /proc."""
    with open("/proc/net/route") as fh:
        for line in fh:
            fields = line.strip().split()
            if fields[1] != '00000000' or not int(fields[3], 16) & 2:
                # If not default route or not RTF_GATEWAY, skip it
                continue

            addr = socket.inet_ntoa(struct.pack("<L", int(fields[2], 16)))
            return {'statusCode': 200, 'body': {'host': addr}}
