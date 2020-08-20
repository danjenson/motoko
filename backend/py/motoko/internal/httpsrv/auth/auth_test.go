package auth

import (
	"fmt"
	"io/ioutil"
	"net/http"
	"net/http/httptest"
	"regexp"
	"testing"

	"github.com/danjenson/motoko/motoko/internal/acctmgr"
	"github.com/danjenson/motoko/motoko/internal/httpsrv/mocks"
	"github.com/gorilla/mux"
	"github.com/markbates/goth"
	"github.com/markbates/goth/gothic"
	"github.com/markbates/goth/providers/faux"
	"github.com/stretchr/testify/assert"
)

func TestHTTPAuth(t *testing.T) {
	email, name := "motoko.kusanagi@gmail.com", "Motoko Kusanagi"
	apiKey := acctmgr.APIKey("12345")
	a := assert.New(t)
	am := &mocks.AccountManager{}
	provider := &faux.Provider{}
	httpAuth := New("dummy_session_key", am, provider)
	router := mux.NewRouter()
	httpAuth.AddAuthenticationHandlers(router)
	secure := func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprint(w, "secure")
	}
	router.HandleFunc("/secure", httpAuth.Authorize(secure))
	ts := httptest.NewTLSServer(router)
	defer ts.Close()
	client := ts.Client()
	// tells client not to follow redirects
	client.CheckRedirect = func(r *http.Request, via []*http.Request) error {
		return http.ErrUseLastResponse
	}
	realCompleteUserAuth := gothic.CompleteUserAuth
	// use this to avoid mocking core gothic feature
	fauxCompleteUserAuth := func(
		w http.ResponseWriter,
		r *http.Request,
	) (goth.User, error) {
		return goth.User{
			Name:  name,
			Email: email,
		}, nil
	}

	t.Run("/login/faux/callback: (existing account)", func(t *testing.T) {
		gothic.CompleteUserAuth = fauxCompleteUserAuth
		am.On("HasAccount", email).Return(true, nil).Once()
		res, _ := client.Get(ts.URL + "/login/faux/callback")
		loc, err := res.Location()
		a.Nil(err, "extracting location")
		a.Regexp(regexp.MustCompile(ts.URL), loc, "redirecting home")
		am.AssertExpectations(t)
	})

	t.Run("/login/faux/callback: (new account)", func(t *testing.T) {
		gothic.CompleteUserAuth = fauxCompleteUserAuth
		am.On("HasAccount", email).Return(false, nil).Once()
		am.On("NewAccount", email, name).Return(apiKey, nil)
		res, _ := client.Get(ts.URL + "/login/faux/callback")
		loc, err := res.Location()
		a.Nil(err, "extracting location")
		a.Regexp(regexp.MustCompile(ts.URL), loc, "redirecting home")
		am.AssertExpectations(t)
	})

	t.Run("/login/faux (existing session)", func(t *testing.T) {
		gothic.CompleteUserAuth = fauxCompleteUserAuth
		res, _ := client.Get(ts.URL + "/login/faux")
		loc, err := res.Location()
		a.Nil(err, "extracting location")
		a.Regexp(regexp.MustCompile(ts.URL), loc, "redirecting to auth")
	})

	t.Run("/login/faux (non-existing session)", func(t *testing.T) {
		gothic.CompleteUserAuth = realCompleteUserAuth
		res, _ := client.Get(ts.URL + "/login/faux")
		loc, err := res.Location()
		a.Nil(err, "extracting location")
		a.Regexp(regexp.MustCompile("example.com"), loc, "redirecting to auth")
	})

	t.Run("/logout/faux", func(t *testing.T) {
		res, _ := client.Get(ts.URL + "/logout/faux")
		loc, err := res.Location()
		a.Nil(err, "extracting location")
		a.Regexp(regexp.MustCompile(ts.URL), loc, "redirecting home")
	})

	t.Run("Authorize and User", func(t *testing.T) {

		// fails on new session
		req, err := http.NewRequest("GET", ts.URL+"/secure", nil)
		a.Nil(err, "creating request")
		res, _ := client.Do(req)
		unauthGoHome(a, res, ts.URL)

		// get credentials should fail
		_, err = httpAuth.User(req)
		a.NotNil(err, "fetching user on new request should fail")

		// simulate login; attach cookie to request
		gothic.CompleteUserAuth = fauxCompleteUserAuth
		am.On("HasAccount", email).Return(true, nil).Once()
		res, _ = client.Get(ts.URL + "/login/faux/callback")
		cookie := res.Cookies()[0]
		req.AddCookie(cookie)

		// should be authorized now
		res, _ = client.Do(req)
		text, err := ioutil.ReadAll(res.Body)
		res.Body.Close()
		a.Equal("secure", string(text))

		// test user; need to use httptest because it adds context
		req = httptest.NewRequest("GET", ts.URL+"/secure", nil)
		req.AddCookie(cookie)
		user, err := httpAuth.User(req)
		a.Nil(err, "fetching user on after login should pass")
		a.Equal(email, user.Email, "emails not equal")

		am.AssertExpectations(t)
	})
}

func unauthGoHome(a *assert.Assertions, res *http.Response, url string) {
	loc, err := res.Location()
	a.Nil(err, "extracting location")
	a.Regexp(regexp.MustCompile(url), loc, "redirecting home")
	a.Equal(res.StatusCode, http.StatusUnauthorized)
}
