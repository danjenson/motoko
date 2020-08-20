DROP TABLE IF EXISTS service_use;
DROP TABLE IF EXISTS api;
DROP TABLE IF EXISTS accounts;

CREATE TABLE accounts (
  email VARCHAR(100) PRIMARY KEY,
  name VARCHAR(100) NOT NULL
);

CREATE TABLE api (
  k VARCHAR(36) PRIMARY KEY,
  email VARCHAR(100),
  name VARCHAR(100) NOT NULL,
  active BOOLEAN NOT NULL,
  FOREIGN KEY (email) REFERENCES accounts(email)
);

CREATE TABLE service_use (
  dt DATETIME NOT NULL,
  api_key VARCHAR(36),
  learn_key VARCHAR(256),
  service ENUM('learn', 'predict'), 
  n_bytes BIGINT UNSIGNED NOT NULL,
  FOREIGN KEY (api_key) REFERENCES api(k)
);


CREATE USER IF NOT EXISTS 'account_manager'@'localhost';
GRANT SELECT ON accounts TO 'account_manager'@'localhost';
GRANT SELECT ON api TO 'account_manager'@'localhost';
GRANT SELECT ON service_use TO 'account_manager'@'localhost';


DELIMITER //


DROP PROCEDURE IF EXISTS activate_api_key;
CREATE PROCEDURE activate_api_key (
  email VARCHAR(100),
  api_key VARCHAR(36)
) MODIFIES SQL DATA
UPDATE api SET active = TRUE WHERE api.email = email AND api.k = api_key;
GRANT EXECUTE ON PROCEDURE activate_api_key TO 'account_manager'@'localhost';


DROP PROCEDURE IF EXISTS deactivate_api_key;
CREATE PROCEDURE deactivate_api_key (
  email VARCHAR(100),
  api_key VARCHAR(36)
) MODIFIES SQL DATA
UPDATE api SET active = FALSE WHERE api.email = email AND api.k = api_key;
GRANT EXECUTE ON PROCEDURE deactivate_api_key TO 'account_manager'@'localhost';


DROP PROCEDURE IF EXISTS default_api_key;
CREATE PROCEDURE default_api_key (
  IN email VARCHAR(100),
  OUT api_key VARCHAR(36)
)
SELECT k AS api_key FROM api WHERE api.email = email AND name = 'default';
GRANT EXECUTE ON PROCEDURE default_api_key TO 'account_manager'@'localhost';


DROP PROCEDURE IF EXISTS has_account;
CREATE PROCEDURE has_account (
  IN email VARCHAR(100),
  OUT present BOOLEAN
)
SELECT EXISTS(SELECT * FROM accounts WHERE accounts.email = email);
GRANT EXECUTE ON PROCEDURE has_account TO 'account_manager'@'localhost';


DROP PROCEDURE IF EXISTS is_active_api_key;
CREATE PROCEDURE is_active_api_key (
  IN email VARCHAR(100),
  IN api_key VARCHAR(36),
  OUT is_active BOOLEAN
)
SELECT active AS is_active FROM api WHERE api.email = email AND api.k = api_key;
GRANT EXECUTE ON PROCEDURE is_active_api_key TO 'account_manager'@'localhost';


DROP PROCEDURE IF EXISTS list_api_keys;
CREATE PROCEDURE list_api_keys (IN email VARCHAR(100))
SELECT name, k, active FROM api WHERE api.email = email;
GRANT EXECUTE ON PROCEDURE list_api_keys TO 'account_manager'@'localhost';


DROP PROCEDURE IF EXISTS new_api_key;
CREATE PROCEDURE new_api_key (
  IN email VARCHAR(100),
  IN name VARCHAR(100),
  OUT api_key VARCHAR(36)
) MODIFIES SQL DATA
BEGIN
  DECLARE api_key VARCHAR(36) DEFAULT UUID();
  INSERT INTO api VALUES (api_key, email, name, TRUE);
  SELECT api_key;
END //
GRANT EXECUTE ON PROCEDURE new_api_key TO 'account_manager'@'localhost';


DROP PROCEDURE IF EXISTS new_account;
CREATE PROCEDURE new_account (
  IN email VARCHAR(100),
  IN name VARCHAR(100),
  OUT api_key VARCHAR(36) 
) MODIFIES SQL DATA
BEGIN
  INSERT INTO accounts VALUES (email, name);
  CALL new_api_key(email, 'default', @api_key);
END //
GRANT EXECUTE ON PROCEDURE new_account TO 'account_manager'@'localhost';


DROP PROCEDURE IF EXISTS register_api_call;
CREATE PROCEDURE register_api_call (
  api_key VARCHAR(36),
  learn_key VARCHAR(36),
  service ENUM('learn', 'predict'),
  n_bytes BIGINT UNSIGNED
) MODIFIES SQL DATA
INSERT INTO service_use VALUES (NOW(), api_key, learn_key, service, n_bytes);
GRANT EXECUTE ON PROCEDURE register_api_call TO 'account_manager'@'localhost';


DROP PROCEDURE IF EXISTS calculate_usage;
CREATE PROCEDURE calculate_usage (
  IN email VARCHAR(100),
  IN service ENUM('learn', 'predict'),
  IN start_dt DATETIME,
  IN end_dt DATETIME,
  OUT n_bytes BIGINT UNSIGNED
)
SELECT SUM(service_use.n_bytes) AS n_bytes
FROM api
JOIN service_use 
ON api.k = service_use.api_key
AND api.email = email
AND service_use.service = service
AND service_use.dt >= start_dt
AND service_use.dt <= end_dt;
GRANT EXECUTE ON PROCEDURE calculate_usage TO 'account_manager'@'localhost';

//
DELIMITER ;
