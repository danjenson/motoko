package authorizer

import (
	"context"

	"github.com/danjenson/motoko/motoko/internal/acctmgr"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
)

type authorizer struct {
	acctmgr.AccountManager
}

func New(am acctmgr.AccountManager) *authorizer {
	return &authorizer{am}
}

func (a *authorizer) StreamInterceptor() func(
	srv interface{},
	ss grpc.ServerStream,
	info *grpc.StreamServerInfo,
	handler grpc.StreamHandler,
) error {
	return func(
		srv interface{},
		ss grpc.ServerStream,
		info *grpc.StreamServerInfo,
		handler grpc.StreamHandler,
	) error {
		if err := a.authorize(ss.Context()); err != nil {
			return err
		}
		return handler(srv, ss)
	}
}

func (a *authorizer) authorize(ctx context.Context) error {
	md, ok := metadata.FromIncomingContext(ctx)
	if !ok {
		return status.Errorf(codes.Unauthenticated, "no authorization information")
	}
	emails := md["email"]
	if len(emails) < 1 {
		return status.Errorf(codes.Unauthenticated, "no email")
	}
	apiKeys := md["api_key"]
	if len(apiKeys) < 1 {
		return status.Errorf(codes.Unauthenticated, "no API key")
	}
	email, apiKey := emails[0], acctmgr.APIKey(apiKeys[0])
	active, err := a.IsActiveAPIKey(email, apiKey)
	if err != nil || !active {
		return status.Errorf(codes.Unauthenticated, "invalid API key")
	}
	return nil
}
