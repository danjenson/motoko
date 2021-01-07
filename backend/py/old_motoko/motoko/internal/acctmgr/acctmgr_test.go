package acctmgr

import (
	"bytes"
	"database/sql"
	"log"
	"os/exec"
	"testing"
	"time"

	_ "github.com/go-sql-driver/mysql"
	"github.com/google/uuid"
	"github.com/stretchr/testify/assert"
)

func init() {
	// (2019-11-18) clear and initialize the database; run this as a system
	// command because database/sql doesn't support multiple statements
	cmd := exec.Command(
		"/usr/bin/mysql",
		"-u", "root",
		"motoko_test",
		"-e", "source init.sql",
	)
	var stderr bytes.Buffer
	cmd.Stderr = &stderr
	if err := cmd.Run(); err != nil {
		log.Fatalf("%s: %s", err, stderr.String())
	}
}

func TestAccountManager(t *testing.T) {
	const email, name = "motoko.kusanagi@sector9.jp", "Motoko Kusanagi"
	a := assert.New(t)

	db, err := sql.Open("mysql", "account_manager@/motoko_test")
	if err != nil {
		t.Fatalf("Error connecting to motoko_test db: %v", err)
	}

	am := New(db)

	// create new account
	defaultAPIKey, err := am.NewAccount(email, name)
	a.Nil(err)
	shouldBeActive := true
	isActive(a, am, email, defaultAPIKey, shouldBeActive)

	// has account
	hasAccount, err := am.HasAccount(email)
	a.True(hasAccount)

	// doesn't have account
	hasAccount, err = am.HasAccount("no.account@google.com")
	a.False(hasAccount)

	// deactivate API key
	err = am.DeactivateAPIKey(email, defaultAPIKey)
	a.Nil(err, "deactive API key failed")
	shouldBeActive = false
	isActive(a, am, email, defaultAPIKey, shouldBeActive)

	// activate API key
	err = am.ActivateAPIKey(email, defaultAPIKey)
	a.Nil(err, "activate API key failed")
	shouldBeActive = true
	isActive(a, am, email, defaultAPIKey, shouldBeActive)

	// check default API key
	defaultAPIKeyRet, err := am.DefaultAPIKey(email)
	a.Nil(err, "retreiving default API key")
	a.Equal(defaultAPIKey, defaultAPIKeyRet, "actual != returned")

	// issue new API key
	newAPIKey, err := am.NewAPIKey(email, "laughing_man")
	shouldBeActive = true
	isActive(a, am, email, newAPIKey, shouldBeActive)

	// list API keys
	records, err := am.ListAPIKeys(email)
	a.Nil(err, "listing API keys")
	a.Len(records, 2)

	// register API calls
	learnerKey := uuid.New().String()
	learnBytes, predictBytes := uint64(1e6), uint64(2e6)
	err = am.RegisterAPICall(defaultAPIKey, learnerKey, LEARN, learnBytes)
	a.Nil(err, "error registering learn API call")
	err = am.RegisterAPICall(newAPIKey, learnerKey, PREDICT, predictBytes)
	a.Nil(err, "error registering predict API call")

	// check usage
	loc, err := time.LoadLocation("America/Los_Angeles")
	a.Nil(err, "loading PST time location")
	end := time.Now().In(loc)
	start := end.Add(-time.Second)
	usage, err := am.Use(email, start, end)
	a.Nil(err, "checking usage")
	a.Equal(learnBytes, usage.LearnBytes, "learn bytes: actual != returned")
	a.Equal(predictBytes, usage.PredictBytes, "predict bytes: actual != returned")
}

func isActive(
	a *assert.Assertions,
	am AccountManager,
	email string,
	apiKey APIKey,
	shouldBeActive bool,
) {
	isActive, err := am.IsActiveAPIKey(email, apiKey)
	a.Nil(err, "is active API key check failed")
	a.False(shouldBeActive && !isActive, "API key should be active")
	a.False(!shouldBeActive && isActive, "API key should not be active")
}
