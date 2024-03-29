DROP TABLE IF EXISTS "user_application_permissions" cascade;
DROP TABLE IF EXISTS "application_users" cascade;

DROP TABLE IF EXISTS "users" cascade;
DROP TABLE IF EXISTS "emails" cascade;

DROP TABLE IF EXISTS "service_account_application_permissions" cascade;
DROP TABLE IF EXISTS "application_service_accounts" cascade;

DROP TABLE IF EXISTS "service_accounts" cascade;

DROP TABLE IF EXISTS "application_permissions" cascade;
DROP TABLE IF EXISTS "application_configs" cascade;
DROP TABLE IF EXISTS "applications" cascade;

DROP FUNCTION IF EXISTS "config_notify" cascade;

DROP TABLE IF EXISTS "config" cascade;

DROP FUNCTION IF EXISTS "trigger_set_timestamp" cascade;
