package credentials

import (
	"context"
	"testing"

	"github.com/danjenson/motoko/motoko/internal/acctmgr"
	"github.com/stretchr/testify/assert"
)

func TestCredentials(t *testing.T) {
	a := assert.New(t)
	email, apiKey := "motoko.kusanagi@sector9.jp", acctmgr.APIKey("12345")
	creds := Credentials{Email: email, APIKey: apiKey}
	valid := map[string]string{"email": email, "api_key": string(apiKey)}
	ctx := context.Background()
	ret, err := creds.GetRequestMetadata(ctx)
	a.Nil(err, "retreiving request metadata")
	a.Equal(ret, valid)
	a.True(creds.RequireTransportSecurity())
}
