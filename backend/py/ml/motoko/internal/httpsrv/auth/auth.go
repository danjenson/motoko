package auth

import (
	"fmt"
	"net/http"

	"github.com/danjenson/motoko/motoko/internal/acctmgr"
	"github.com/gorilla/mux"
	"github.com/gorilla/sessions"
	"github.com/markbates/goth"
	"github.com/markbates/goth/gothic"
)

type Auth interface {
	AddAuthenticationHandlers(r *mux.Router) error
	Authorize(h http.HandlerFunc) http.HandlerFunc
	User(r *http.Request) (goth.User, error)
}

type auth struct {
	store *sessions.CookieStore
	acctmgr.AccountManager
}

func New(
	sessionKey string,
	am acctmgr.AccountManager,
	providers ...goth.Provider,
) *auth {
	goth.UseProviders(providers...)
	store := sessions.NewCookieStore([]byte(sessionKey))
	// use same store for client and OAuth2 sessions
	gothic.Store = store
	return &auth{store, am}
}

func (a auth) AddAuthenticationHandlers(r *mux.Router) error {
	r.HandleFunc(
		"/login/{provider}/callback",
		func(w http.ResponseWriter, r *http.Request) {
			user, err := gothic.CompleteUserAuth(w, r)
			if err != nil {
				http.Error(w, err.Error(), http.StatusInternalServerError)
				return
			}
			s, err := a.store.Get(r, "session-name")
			if err != nil {
				http.Error(w, err.Error(), http.StatusInternalServerError)
				return
			}
			if user.Email == "" {
				http.Error(w, "email address not provided", http.StatusUnauthorized)
				return
			}
			hasAccount, err := a.HasAccount(user.Email)
			if err != nil {
				http.Error(w, err.Error(), http.StatusInternalServerError)
				return
			}
			if !hasAccount {
				_, err = a.NewAccount(user.Email, user.Name)
				if err != nil {
					http.Error(w, err.Error(), http.StatusInternalServerError)
					return
				}
			}
			s.Values["user"] = user
			err = s.Save(r, w)
			if err != nil {
				http.Error(w, err.Error(), http.StatusInternalServerError)
				return
			}
			goHome(w)
		},
	)
	r.HandleFunc(
		"/login/{provider}",
		func(w http.ResponseWriter, r *http.Request) {
			// check if already logged in
			if _, err := gothic.CompleteUserAuth(w, r); err == nil {
				goHome(w)
				return
			}
			gothic.BeginAuthHandler(w, r)
		},
	)
	r.HandleFunc(
		"/logout/{provider}",
		func(w http.ResponseWriter, r *http.Request) {
			gothic.Logout(w, r)
			goHome(w)
		},
	)
	return nil
}

func goHome(w http.ResponseWriter) {
	w.Header().Set("Location", "/")
	w.WriteHeader(http.StatusTemporaryRedirect)
}

func goLogin(w http.ResponseWriter) {
	w.Header().Set("Location", "/")
	w.WriteHeader(http.StatusUnauthorized)
}

func (a auth) Authorize(h http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		s, err := a.store.Get(r, "session-name")
		if s.IsNew || err != nil {
			goLogin(w)
			return
		}
		h(w, r)
	}
}

func (a auth) User(r *http.Request) (goth.User, error) {
	user := goth.User{}
	s, err := a.store.Get(r, "session-name")
	if err != nil {
		return user, err
	}
	if s.IsNew {
		return user, fmt.Errorf("no session")
	}
	user = s.Values["user"].(goth.User)
	return user, nil
}
