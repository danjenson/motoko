package httpsrv

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/http/httptest"
	"net/url"
	"path/filepath"
	"testing"

	"github.com/danjenson/motoko/motoko/internal/acctmgr"
	"github.com/danjenson/motoko/motoko/internal/httpsrv/mocks"
	"github.com/gorilla/mux"
	"github.com/markbates/goth"
	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

func TestHTTPServer(t *testing.T) {
	a := assert.New(t)

	// resources
	apiKey := acctmgr.APIKey("12345")
	email, name := "motoko.kusanagi@sector9.jp", "Motoko Kusanagi"
	user := goth.User{Email: email, Name: name}
	webRoot := filepath.Join("testdata", "webroot")
	auth := &mocks.Auth{}
	am := &mocks.AccountManager{}
	router := mux.NewRouter()
	s := &server{auth, am, router}
	ts := httptest.NewTLSServer(s)
	client := ts.Client()

	isGRPC := func(r *http.Request) bool {
		if r.Header.Get("Content-Type") != "application/grpc-web" {
			return false
		}
		return true
	}

	var grpcHandlerFunc http.HandlerFunc = func(
		w http.ResponseWriter,
		r *http.Request,
	) {
		fmt.Fprint(w, "grpc")
	}

	// trust that authorization works in auth package
	auth.On("AddAuthenticationHandlers", s.router).Return(nil).Once()
	auth.On("Authorize", mock.Anything).Return(grpcHandlerFunc).Once()
	auth.On("Authorize", mock.Anything).Return(s.handleAPIKey()).Once()
	s.routes(webRoot, isGRPC, grpcHandlerFunc)
	auth.AssertExpectations(t)

	t.Run("/api_key/new", func(t *testing.T) {
		auth.On("User", mock.Anything).Return(user, nil).Once()
		newKeyName := "apple"
		am.On("NewAPIKey", email, newKeyName).Return(apiKey, nil)
		data := url.Values{}
		data.Set("name", newKeyName)
		res, err := client.PostForm(ts.URL+"/api_key/new", data)
		a.Nil(err, "/api_key/new")
		apiKeyRet := acctmgr.APIKey(res.Header.Get("Authorization"))
		a.Equal(apiKey, apiKeyRet)
		auth.AssertExpectations(t)
		am.AssertExpectations(t)
	})

	t.Run("/api_key/list", func(t *testing.T) {
		newKeyName := "apple"
		auth.On("User", mock.Anything).Return(user, nil).Once()
		records := []acctmgr.APIKeyRecord{
			{Name: newKeyName, Key: apiKey, Active: true},
		}
		am.On("ListAPIKeys", email).Return(records, nil)
		res, err := client.PostForm(ts.URL+"/api_key/list", url.Values{})
		a.Nil(err, "/api_key/list")
		body, err := ioutil.ReadAll(res.Body)
		a.Nil(err, "/api_key/list reading body")
		defer res.Body.Close()
		recordsRet := []acctmgr.APIKeyRecord{}
		err = json.Unmarshal(body, &recordsRet)
		a.Nil(err, "/api_key/list deserialize body")
		a.Equal(records, recordsRet, "API Key records not equivalent")
		auth.AssertExpectations(t)
		am.AssertExpectations(t)
	})

	t.Run("/api_key/activate", func(t *testing.T) {
		auth.On("User", mock.Anything).Return(user, nil).Twice()
		apiKeyErr := fmt.Errorf("bad API key")
		am.On("ActivateAPIKey", email, apiKey).Return(nil).Once()
		req, err := http.NewRequest("POST", ts.URL+"/api_key/activate", nil)
		a.Nil(err, "making request")
		req.Header.Add("Authorization", string(apiKey))
		a.Nil(err, "/api_key/activate creating new request")
		res, err := client.Do(req)
		a.Nil(err, "/api_key/activate do first request")
		a.Equal(res.StatusCode, http.StatusOK)
		am.On("ActivateAPIKey", email, apiKey).Return(apiKeyErr).Once()
		res, err = client.Do(req)
		a.Nil(err, "/api_key/activate do second request")
		a.Equal(res.StatusCode, http.StatusBadRequest)
		auth.AssertExpectations(t)
		am.AssertExpectations(t)
	})

	t.Run("/api_key/deactivate", func(t *testing.T) {
		auth.On("User", mock.Anything).Return(user, nil).Twice()
		apiKeyErr := fmt.Errorf("bad API key")
		am.On("DeactivateAPIKey", email, apiKey).Return(nil).Once()
		req, err := http.NewRequest("POST", ts.URL+"/api_key/deactivate", nil)
		a.Nil(err, "making request")
		req.Header.Add("Authorization", string(apiKey))
		a.Nil(err, "/api_key/deactivate creating new request")
		res, _ := client.Do(req)
		a.Equal(res.StatusCode, http.StatusOK)
		am.On("DeactivateAPIKey", email, apiKey).Return(apiKeyErr).Once()
		res, _ = client.Do(req)
		a.Equal(res.StatusCode, http.StatusBadRequest)
		auth.AssertExpectations(t)
		am.AssertExpectations(t)
	})

	t.Run("grpc", func(t *testing.T) {
		req, err := http.NewRequest("POST", ts.URL, nil)
		a.Nil(err, "making request")
		req.Header.Set("Content-Type", "application/grpc-web")
		res, _ := client.Do(req)
		a.Equal(res.StatusCode, http.StatusOK)
		text, err := ioutil.ReadAll(res.Body)
		a.Nil(err, "reading body of grpc response")
		res.Body.Close()
		a.Equal("grpc", string(text))
		auth.AssertExpectations(t)
	})

	t.Run("/", func(t *testing.T) {
		res, err := client.Get(ts.URL)
		a.Nil(err, "serving home")
		a.Equal(http.StatusOK, res.StatusCode)
		page, err := ioutil.ReadFile(filepath.Join(webRoot, "index.html"))
		a.Nil(err, "reading page")
		pageRet, err := ioutil.ReadAll(res.Body)
		a.Nil(err, "reading returned page")
		res.Body.Close()
		a.Equal(page, pageRet)
	})
}
