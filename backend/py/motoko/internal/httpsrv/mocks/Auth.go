// Code generated by mockery v1.0.0. DO NOT EDIT.

package mocks

import goth "github.com/markbates/goth"
import http "net/http"
import mock "github.com/stretchr/testify/mock"
import mux "github.com/gorilla/mux"

// Auth is an autogenerated mock type for the Auth type
type Auth struct {
	mock.Mock
}

// AddAuthenticationHandlers provides a mock function with given fields: r
func (_m *Auth) AddAuthenticationHandlers(r *mux.Router) error {
	ret := _m.Called(r)

	var r0 error
	if rf, ok := ret.Get(0).(func(*mux.Router) error); ok {
		r0 = rf(r)
	} else {
		r0 = ret.Error(0)
	}

	return r0
}

// Authorize provides a mock function with given fields: h
func (_m *Auth) Authorize(h http.HandlerFunc) http.HandlerFunc {
	ret := _m.Called(h)

	var r0 http.HandlerFunc
	if rf, ok := ret.Get(0).(func(http.HandlerFunc) http.HandlerFunc); ok {
		r0 = rf(h)
	} else {
		if ret.Get(0) != nil {
			r0 = ret.Get(0).(http.HandlerFunc)
		}
	}

	return r0
}

// User provides a mock function with given fields: r
func (_m *Auth) User(r *http.Request) (goth.User, error) {
	ret := _m.Called(r)

	var r0 goth.User
	if rf, ok := ret.Get(0).(func(*http.Request) goth.User); ok {
		r0 = rf(r)
	} else {
		r0 = ret.Get(0).(goth.User)
	}

	var r1 error
	if rf, ok := ret.Get(1).(func(*http.Request) error); ok {
		r1 = rf(r)
	} else {
		r1 = ret.Error(1)
	}

	return r0, r1
}
