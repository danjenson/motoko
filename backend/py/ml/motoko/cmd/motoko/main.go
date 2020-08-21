package main

import (
	"context"
	"database/sql"
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"net"
	"net/http"
	"os"
	"os/signal"
	"path/filepath"
	"time"

	"github.com/danjenson/motoko/motoko/internal/acctmgr"
	"github.com/danjenson/motoko/motoko/internal/grpcsrv"
	"github.com/danjenson/motoko/motoko/internal/grpcsrv/authorizer"
	"github.com/danjenson/motoko/motoko/internal/httpsrv"
	"github.com/danjenson/motoko/motoko/internal/httpsrv/auth"
	"github.com/danjenson/motoko/motoko/internal/pb"
	_ "github.com/go-sql-driver/mysql"
	"github.com/improbable-eng/grpc-web/go/grpcweb"
	"github.com/markbates/goth/providers/google"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
)

func main() {
	var (
		webRoot    = flag.String("webroot", "", "root directory of website")
		sessionKey = flag.String("session_key", "", "for managing user sessions")
		gKey       = flag.String("google_key", "", "google oath client id")
		gSecret    = flag.String("google_secret", "", "google oath client secret")
		tlsCert    = flag.String("tls_cert", "", "public TLS key, i.e. server.crt")
		tlsKey     = flag.String("tls_key", "", "private TLS key, i.e. server.key")
		mPort      = flag.Uint("motoko_port", 9000, "motoko gRPC server port")
		tkPort     = flag.Uint("tachikoma_port", 9001, "tachikoma gRPC server port")
		database   = flag.String("database", "motoko", "sql database to use")
	)
	flag.Parse()
	log.SetFlags(0)
	log.SetOutput(&logger{})

	mAddress := fmt.Sprintf("localhost:%d", *mPort)
	tkAddress := fmt.Sprintf("localhost:%d", *tkPort)
	motd, err := ioutil.ReadFile(filepath.Join("cmd", "motoko", "motd.txt"))
	failIf(err, "reading motd.txt")
	tls, err := credentials.NewServerTLSFromFile(*tlsCert, *tlsKey)
	failIf(err, "reading tls cert for gRPC")
	db, err := sql.Open("mysql", "account_manager@/"+*database)
	defer db.Close()
	failIf(err, "connecting to database "+*database)
	am := acctmgr.New(db)
	tkConn, err := grpc.Dial(tkAddress, grpc.WithInsecure(), grpc.WithBlock())
	defer tkConn.Close()
	tachikoma := pb.NewTachikomaClient(tkConn)
	motoko := grpcsrv.New(am, tachikoma)
	authz := authorizer.New(am)

	grpcFail := make(chan error)
	grpcServer := grpc.NewServer(
		grpc.StreamInterceptor(authz.StreamInterceptor()),
		grpc.Creds(tls),
	)
	pb.RegisterMotokoServer(grpcServer, motoko)
	go func() {
		listener, err := net.Listen("tcp", mAddress)
		if err != nil {
			grpcFail <- err
			return
		}
		log.Println("gRPC server started at " + mAddress)
		grpcFail <- grpcServer.Serve(listener)
	}()
	wgrpcServer := grpcweb.WrapServer(
		grpcServer,
		grpcweb.WithWebsockets(true),
		grpcweb.WithWebsocketOriginFunc(func(r *http.Request) bool { return true }),
	)
	gp := google.New(*gKey, *gSecret, "https://localhost/auth/google/callback")
	authn := auth.New(*sessionKey, am, gp)
	httpHandler := httpsrv.New(
		*webRoot,
		authn,
		am,
		wgrpcServer.IsGrpcWebSocketRequest,
		wgrpcServer.ServeHTTP,
	)
	httpFail := make(chan error)
	httpServer := &http.Server{Addr: "localhost:443", Handler: httpHandler}
	go func() {
		log.Println("HTTP server started at localhost:443")
		httpFail <- httpServer.ListenAndServeTLS(*tlsCert, *tlsKey)
	}()

	log.Println(string(motd))
	stop := make(chan os.Signal)
	signal.Notify(stop, os.Interrupt)

	ctx := context.Background()
	select {
	case grpcFailure := <-grpcFail:
		log.Fatalln(grpcFailure)
	case httpFailure := <-httpFail:
		log.Fatalln(httpFailure)
		// TODO(danj): once CA is added, fail here
	case <-stop:
		ctx, cancel := context.WithTimeout(ctx, 5*time.Second)
		defer cancel()
		if err := httpServer.Shutdown(ctx); err != nil {
			log.Println(err)
		}
		grpcServer.GracefulStop()
		log.Println("stopping")
	}
}

type logger struct{}

func (w logger) Write(bytes []byte) (int, error) {
	t := time.Now().Format("01/02/2006 03:04:05 PM")
	prefix := fmt.Sprintf("%s [PID %d]: ", t, os.Getpid())
	return fmt.Print(prefix + string(bytes))
}

func failIf(err error, msg string) {
	if err != nil {
		log.Fatalf("error "+msg+": %v", err)
	}
}
