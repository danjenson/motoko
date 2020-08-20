package credentials

import (
	"context"

	"github.com/danjenson/motoko/motoko/internal/acctmgr"
)

type Credentials struct {
	Email  string
	APIKey acctmgr.APIKey
}

func (c Credentials) GetRequestMetadata(
	ctx context.Context,
	uri ...string,
) (map[string]string, error) {
	return map[string]string{
		"email":   c.Email,
		"api_key": string(c.APIKey),
	}, nil
}

func (Credentials) RequireTransportSecurity() bool {
	return true
}
