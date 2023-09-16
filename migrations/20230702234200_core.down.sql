DROP TABLE IF EXISTS "users" cascade;
DROP TABLE IF EXISTS "emails" cascade;

DROP TABLE IF EXISTS "application_configs" cascade;
DROP TABLE IF EXISTS "applications" cascade;

DROP TABLE IF EXISTS "config" cascade;

DROP FUNCTION IF EXISTS "trigger_set_timestamp" cascade;

DROP EXTENSION IF EXISTS "pgcrypto";
