package httpsrv

import (
	"encoding/json"
	"fmt"
	"net/http"

	"github.com/danjenson/motoko/motoko/internal/acctmgr"
	"github.com/danjenson/motoko/motoko/internal/httpsrv/auth"
	"github.com/gorilla/mux"
)

type server struct {
	auth   auth.Auth
	am     acctmgr.AccountManager
	router *mux.Router
}

// handles authentication, authorization, and account management
func New(
	webRoot string,
	auth auth.Auth,
	am acctmgr.AccountManager,
	isGRPC func(r *http.Request) bool,
	grpcHandlerFunc http.HandlerFunc,
) *server {
	router := mux.NewRouter()
	s := &server{auth, am, router}
	s.routes(webRoot, isGRPC, grpcHandlerFunc)
	return s
}

func (s *server) routes(
	webRoot string,
	isGRPC func(r *http.Request) bool,
	grpcHandlerFunc http.HandlerFunc,
) {
	s.auth.AddAuthenticationHandlers(s.router)
	s.router.MatcherFunc(func(r *http.Request, rm *mux.RouteMatch) bool {
		return isGRPC(r)
	}).HandlerFunc(s.addGRPCAuth(grpcHandlerFunc))
	s.router.HandleFunc("/api_key/{action}", s.auth.Authorize(s.handleAPIKey()))
	s.router.PathPrefix("/").HandlerFunc(s.handleWebsite(webRoot))
}

func (s *server) addGRPCAuth(h http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// TODO(danj): add authentication header here
		fmt.Println("TODO: add gRPC auth in HTTP server")
		h(w, r)
	}
}

func (s *server) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	s.router.ServeHTTP(w, r)
}

// pass around API keys in the Authorization header,
// since most loggers do not log this header
func (s *server) handleAPIKey() http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		user, err := s.auth.User(r)
		if err != nil {
			http.Error(w, err.Error(), http.StatusUnauthorized)
			return
		}
		email := user.Email
		r.ParseForm()
		switch action := mux.Vars(r)["action"]; action {
		case "new":
			name := r.Form.Get("name")
			if name == "" {
				http.Error(w, "missing new API key 'name'", http.StatusBadRequest)
				return
			}
			apiKey, err := s.am.NewAPIKey(email, name)
			if err != nil {
				http.Error(w, err.Error(), http.StatusInternalServerError)
				return
			}
			w.Header().Set("Authorization", string(apiKey))
		case "list":
			records, err := s.am.ListAPIKeys(email)
			if err != nil {
				http.Error(w, err.Error(), http.StatusInternalServerError)
				return
			}
			s.respond(w, r, records, http.StatusOK)
		case "activate", "deactivate":
			apiKeyRaw := r.Header.Get("Authorization")
			if apiKeyRaw == "" {
				http.Error(w, "missing API key", http.StatusBadRequest)
				return
			}
			var err error
			apiKey := acctmgr.APIKey(apiKeyRaw)
			if action == "activate" {
				err = s.am.ActivateAPIKey(email, apiKey)
			} else {
				err = s.am.DeactivateAPIKey(email, apiKey)
			}
			if err != nil {
				http.Error(w, "invalid API key", http.StatusBadRequest)
				return
			}
		default:
			http.Error(w, "invalid API key action", http.StatusBadRequest)
		}
	}
}

func (s *server) handleWebsite(webRoot string) http.HandlerFunc {
	return http.FileServer(http.Dir(webRoot)).ServeHTTP
}

func (s *server) respond(
	w http.ResponseWriter,
	r *http.Request,
	data interface{},
	status int,
) {
	w.WriteHeader(status)
	if data != nil {
		err := json.NewEncoder(w).Encode(data)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
	}
}
