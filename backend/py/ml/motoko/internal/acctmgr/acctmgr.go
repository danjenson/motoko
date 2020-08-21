package acctmgr

import (
	"database/sql"
	"fmt"
	"regexp"
	"time"
)

const (
	LEARN ServiceType = iota + 1
	PREDICT
)

type ServiceType int
type APIKey string

type Usage struct {
	LearnBytes   uint64
	PredictBytes uint64
}

type AccountManager interface {
	ActivateAPIKey(email string, apiKey APIKey) error
	DeactivateAPIKey(email string, apiKey APIKey) error
	DefaultAPIKey(email string) (APIKey, error)
	HasAccount(email string) (bool, error)
	IsActiveAPIKey(email string, apiKey APIKey) (bool, error)
	ListAPIKeys(email string) ([]APIKeyRecord, error)
	NewAPIKey(email, name string) (APIKey, error)
	NewAccount(email, name string) (APIKey, error)
	RegisterAPICall(
		apiKey APIKey,
		learnKey string,
		serviceType ServiceType,
		nBytes uint64,
	) error
	Use(email string, start, end time.Time) (Usage, error)
}

type accountManager struct {
	db *sql.DB
}

type APIKeyRecord struct {
	Name   string
	Key    APIKey
	Active bool
}

func New(db *sql.DB) *accountManager {
	return &accountManager{db}
}

func (am *accountManager) ActivateAPIKey(email string, apiKey APIKey) error {
	const query = "CALL activate_api_key(?, ?)"
	if _, err := am.db.Exec(query, email, apiKey); err != nil {
		return err
	}
	return nil
}

func (am *accountManager) DeactivateAPIKey(email string, apiKey APIKey) error {
	const query = "CALL deactivate_api_key(?, ?)"
	if _, err := am.db.Exec(query, email, apiKey); err != nil {
		return err
	}
	return nil
}

func (am *accountManager) DefaultAPIKey(email string) (APIKey, error) {
	const query = "CALL default_api_key(?, @apiKey)"
	var apiKey APIKey
	if err := am.db.QueryRow(query, email).Scan(&apiKey); err != nil {
		return "", err
	}
	return apiKey, nil
}

func (am *accountManager) HasAccount(email string) (bool, error) {
	const query = "CALL has_account(?, @hasAccount)"
	var hasAccount bool
	if err := am.db.QueryRow(query, email).Scan(&hasAccount); err != nil {
		return false, err
	}
	return hasAccount, nil
}

func (am *accountManager) IsActiveAPIKey(
	email string,
	apiKey APIKey,
) (bool, error) {
	const query = "CALL is_active_api_key(?, ?, @isActive)"
	var isActive bool
	if err := am.db.QueryRow(query, email, apiKey).Scan(&isActive); err != nil {
		return false, err
	}
	return isActive, nil
}

func (am *accountManager) ListAPIKeys(email string) ([]APIKeyRecord, error) {
	const query = "CALL list_api_keys(?)"
	var records []APIKeyRecord
	rows, err := am.db.Query(query, email)
	defer rows.Close()
	if err != nil {
		return records, err
	}
	for rows.Next() {
		var (
			name   string
			key    string
			active bool
		)
		if err := rows.Scan(&name, &key, &active); err != nil {
			return records, err
		}
		record := APIKeyRecord{Name: name, Key: APIKey(key), Active: active}
		records = append(records, record)
	}
	return records, nil
}

func (am *accountManager) NewAPIKey(email, name string) (APIKey, error) {
	const query = "CALL new_api_key(?, ?, @apiKey)"
	var apiKey APIKey
	if err := am.db.QueryRow(query, email, name).Scan(&apiKey); err != nil {
		return "", err
	}
	return apiKey, nil
}

func (am *accountManager) NewAccount(email, name string) (APIKey, error) {
	const (
		query   = "CALL new_account(?, ?, @apiKey)"
		reEmail = `^\w+([.-]?\w+)*@\w+([.-]?\w+)*(\.\w{2,4})+$`
		reName  = `[a-zA-Z- .]+$`
	)
	var apiKey APIKey
	if !regexp.MustCompile(reEmail).MatchString(email) {
		return "", fmt.Errorf("invalid email: %s", email)
	}
	if !regexp.MustCompile(reName).MatchString(name) {
		return "", fmt.Errorf("invalid name: %s", name)
	}
	if err := am.db.QueryRow(query, email, name).Scan(&apiKey); err != nil {
		return "", err
	}
	return apiKey, nil
}

func (am *accountManager) RegisterAPICall(
	apiKey APIKey,
	learnKey string,
	serviceType ServiceType,
	nBytes uint64,
) error {
	const query = "CALL register_api_call(?, ?, ?, ?)"
	if _, err := am.db.Exec(
		query,
		apiKey,
		learnKey,
		serviceType,
		nBytes,
	); err != nil {
		return err
	}
	return nil
}

func (am *accountManager) Use(
	email string,
	start time.Time,
	end time.Time,
) (Usage, error) {
	const (
		query  = "CALL calculate_usage(?, ?, ?, ?, @nBytes)"
		isoFmt = "2006-01-02 15:04:05"
	)
	var learnBytes, predictBytes uint64
	s, e := start.Format(isoFmt), end.Format(isoFmt)
	err := am.db.QueryRow(query, email, LEARN, s, e).Scan(&learnBytes)
	usage := Usage{}
	if err != nil {
		return usage, err
	}
	usage.LearnBytes = learnBytes
	err = am.db.QueryRow(query, email, PREDICT, s, e).Scan(&predictBytes)
	if err != nil {
		return usage, err
	}
	usage.PredictBytes = predictBytes
	return usage, nil
}
