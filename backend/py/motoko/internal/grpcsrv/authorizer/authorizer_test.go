package authorizer

import (
	"context"
	"testing"

	"github.com/danjenson/motoko/motoko/internal/acctmgr"
	"github.com/danjenson/motoko/motoko/internal/grpcsrv/authorizer/mocks"
	"github.com/stretchr/testify/assert"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
)

func TestAuthorizer(t *testing.T) {
	a := assert.New(t)

	email, apiKey := "motoko.kusanagi@sector9.jp", acctmgr.APIKey("12345")
	am := &mocks.AccountManager{}
	authorizer := New(am)
	ctx := context.Background()

	// test active
	am.On("IsActiveAPIKey", email, apiKey).Return(true, nil).Once()
	md := map[string]string{"email": email, "api_key": string(apiKey)}
	valid := metadata.New(md)
	validCtx := metadata.NewIncomingContext(ctx, valid)
	err := authorizer.authorize(validCtx)
	a.Nil(err, "valid context should not return an error")

	// test inactive
	am.On("IsActiveAPIKey", email, apiKey).Return(false, nil).Once()
	err = status.Errorf(codes.Unauthenticated, "invalid API key")
	errRet := authorizer.authorize(validCtx)
	a.Equal(err, errRet)

	// missing email
	noEmail := metadata.New(map[string]string{"api_key": string(apiKey)})
	noEmailCtx := metadata.NewIncomingContext(ctx, noEmail)
	err = status.Errorf(codes.Unauthenticated, "no email")
	errRet = authorizer.authorize(noEmailCtx)
	a.Equal(err, errRet)

	// missing API key
	noAPIKey := metadata.New(map[string]string{"email": email})
	noAPIKeyCtx := metadata.NewIncomingContext(ctx, noAPIKey)
	err = status.Errorf(codes.Unauthenticated, "no API key")
	errRet = authorizer.authorize(noAPIKeyCtx)
	a.Equal(err, errRet)

	// empty metadata (same as missing)
	noAuth := metadata.New(map[string]string{})
	noAuthCtx := metadata.NewIncomingContext(ctx, noAuth)
	// first key to be missing is email
	err = status.Errorf(codes.Unauthenticated, "no email")
	errRet = authorizer.authorize(noAuthCtx)
	a.Equal(err, errRet)

	am.AssertExpectations(t)
}
