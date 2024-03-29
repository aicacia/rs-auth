CREATE EXTENSION IF NOT EXISTS "pgcrypto";


CREATE FUNCTION "trigger_set_timestamp"()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = CURRENT_TIMESTAMP;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TABLE "config" (
	"key" VARCHAR(255) NOT NULL PRIMARY KEY,
	"value" JSONB NOT NULL DEFAULT 'null',
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX "config_unique_idx" ON "config" ("key");
CREATE TRIGGER "config_set_timestamp" BEFORE UPDATE ON "config" FOR EACH ROW EXECUTE PROCEDURE "trigger_set_timestamp"();

INSERT INTO "config" ("key", "value") VALUES
  ('server.address', '"0.0.0.0"'),
  ('server.port', '8080'),
  ('server.uri', '"http://localhost:8080"'),
  ('log_level', '"debug"');


CREATE FUNCTION config_notify() RETURNS trigger AS $$
DECLARE
  "key" VARCHAR(255);
  "value" JSONB;
BEGIN
  IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
  "key" = NEW."key";
  ELSE
  "key" = OLD."key";
  END IF;
  IF TG_OP != 'UPDATE' OR NEW."value" != OLD."value" THEN
  PERFORM pg_notify('config_channel', json_build_object('table', TG_TABLE_NAME, 'key', "key", 'value', NEW."value", 'action_type', TG_OP)::text);
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER "config_notify_update" AFTER UPDATE ON "config" FOR EACH ROW EXECUTE PROCEDURE config_notify();
CREATE TRIGGER "config_notify_insert" AFTER INSERT ON "config" FOR EACH ROW EXECUTE PROCEDURE config_notify();
CREATE TRIGGER "config_notify_delete" AFTER DELETE ON "config" FOR EACH ROW EXECUTE PROCEDURE config_notify();


CREATE TABLE "applications" (
	"id" SERIAL PRIMARY KEY,
  "description" VARCHAR(255) NOT NULL,
  "uri" VARCHAR(255) NOT NULL,
  "secret" VARCHAR(255) DEFAULT encode(gen_random_bytes(127), 'hex') NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX "applications_uri_unique_idx" ON "applications" ("uri");
CREATE TRIGGER "applications_set_timestamp" BEFORE UPDATE ON "applications" FOR EACH ROW EXECUTE PROCEDURE "trigger_set_timestamp"();

INSERT INTO "applications" ("description", "uri") VALUES
  ('Admin', 'admin');

INSERT INTO "config" ("key", "value") VALUES
  ('admin_application_id', (SELECT to_jsonb(id) FROM "applications" WHERE uri='admin' LIMIT 1));


CREATE TABLE "application_configs" (
  "application_id" INT4 NOT NULL,
	"key" VARCHAR(255) NOT NULL,
	"value" JSONB NOT NULL DEFAULT 'null',
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT "application_configs_application_id_fk" FOREIGN KEY("application_id") REFERENCES "applications"("id") ON DELETE CASCADE,
  PRIMARY KEY("application_id", "key")
);
CREATE TRIGGER "application_configs_set_timestamp" BEFORE UPDATE ON "application_configs" FOR EACH ROW EXECUTE PROCEDURE "trigger_set_timestamp"();

INSERT INTO "application_configs" ("application_id", "key", "value") VALUES
  ((SELECT id FROM "applications" WHERE uri='admin' LIMIT 1), 'jwt.secret', to_jsonb(encode(public.gen_random_bytes(255), 'base64'))),
  ((SELECT id FROM "applications" WHERE uri='admin' LIMIT 1), 'jwt.expires_in_seconds', '86400'),
  ((SELECT id FROM "applications" WHERE uri='admin' LIMIT 1), 'uri', '"http://localhost:5173"'),
  ((SELECT id FROM "applications" WHERE uri='admin' LIMIT 1), 'signin.password', 'true'),
  ((SELECT id FROM "applications" WHERE uri='admin' LIMIT 1), 'signup.enabled', 'false'),
  ((SELECT id FROM "applications" WHERE uri='admin' LIMIT 1), 'signup.password', 'false');


CREATE TABLE "application_permissions"(
	"id" SERIAL PRIMARY KEY,
	"application_id" INT4 NOT NULL,
	"description" VARCHAR(255) NOT NULL,
	"uri" VARCHAR(255) NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT "application_permissions_application_id_fk" FOREIGN KEY("application_id") REFERENCES "applications"("id") ON DELETE CASCADE
);
CREATE UNIQUE INDEX "application_permissions_application_id_uri_unique_idx" ON "application_permissions" ("application_id", "uri");

INSERT INTO "application_permissions" ("application_id", "description", "uri")
  VALUES
    ((SELECT id FROM "applications" WHERE uri='admin' LIMIT 1), 'Admin', 'admin');


CREATE TABLE "service_accounts"(
	"id" SERIAL PRIMARY KEY,
	"description" VARCHAR(255) NOT NULL,
  "uri" VARCHAR(255) NOT NULL,
	"encrypted_secret" VARCHAR(255) NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX "service_accounts_uri_unique_idx" ON "applications" ("uri");
CREATE TRIGGER "service_accounts_set_timestamp" BEFORE UPDATE ON "service_accounts" FOR EACH ROW EXECUTE PROCEDURE "trigger_set_timestamp"();


CREATE TABLE "application_service_accounts"(
	"application_id" INT4 NOT NULL,
	"service_account_id" INT4 NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT "application_service_accounts_application_id_fk" FOREIGN KEY("application_id") REFERENCES "applications"("id") ON DELETE CASCADE,
  CONSTRAINT "application_service_accounts_service_account_id_fk" FOREIGN KEY("service_account_id") REFERENCES "service_accounts"("id") ON DELETE CASCADE
);


CREATE TABLE "service_account_application_permissions"(
	"service_account_id" INT4 NOT NULL,
	"application_permission_id" INT4 NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT "service_account_application_permissions_service_account_id_fk" FOREIGN KEY("service_account_id") REFERENCES "service_accounts"("id") ON DELETE CASCADE,
  CONSTRAINT "service_account_application_permissions_application_permission_id_fk" FOREIGN KEY("application_permission_id") REFERENCES "application_permissions"("id") ON DELETE CASCADE,
  PRIMARY KEY("service_account_id", "application_permission_id")
);


CREATE TABLE "users"(
	"id" SERIAL PRIMARY KEY,
	"username" VARCHAR(255) NOT NULL,
	"encrypted_password" VARCHAR(255) NOT NULL,
  "reset_password_token" UUID,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE UNIQUE INDEX "users_username_unique_idx" ON "users" ("username");
CREATE UNIQUE INDEX "users_reset_password_token_unique_idx" ON "users" ("reset_password_token");
CREATE TRIGGER "users_set_timestamp" BEFORE UPDATE ON "users" FOR EACH ROW EXECUTE PROCEDURE "trigger_set_timestamp"();

INSERT INTO "users" ("username", "encrypted_password")
  VALUES ('admin', '$argon2id$v=19$m=16384,t=8,p=8$eOoEbWBjLe3a+03wPkO8hFYaJjXH/5x7TwjnhhQpHj0$Z1hXFJsioMJlakcP2NGCpukpKZcJagB9dpB6aPWHUxg');


CREATE TABLE "emails"(
	"id" SERIAL PRIMARY KEY,
	"user_id" INT4 NOT NULL,
	"email" VARCHAR(255) NOT NULL,
  "confirmed" boolean NOT NULL DEFAULT false,
  "confirmation_token" UUID,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT "emails_user_id_fk" FOREIGN KEY("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);
CREATE UNIQUE INDEX "emails_email_unique_idx" ON "emails" ("email");
CREATE UNIQUE INDEX "emails_confirmation_token_unique_idx" ON "emails" ("confirmation_token");
CREATE TRIGGER "emails_set_timestamp" BEFORE UPDATE ON "emails" FOR EACH ROW EXECUTE PROCEDURE "trigger_set_timestamp"();

ALTER TABLE "users" ADD COLUMN "email_id" INT4;
ALTER TABLE "users" ADD CONSTRAINT "users_email_id_fk" FOREIGN KEY("email_id") REFERENCES "emails"("id") ON DELETE CASCADE;
CREATE UNIQUE INDEX "users_email_id_unique_idx" ON "users" ("email_id");


CREATE TABLE "application_users"(
	"application_id" INT4 NOT NULL,
	"user_id" INT4 NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT "application_users_application_id_fk" FOREIGN KEY("application_id") REFERENCES "applications"("id") ON DELETE CASCADE,
  CONSTRAINT "application_users_user_id_fk" FOREIGN KEY("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);

INSERT INTO "application_users" ("application_id", "user_id")
  VALUES ((SELECT id FROM "applications" WHERE uri='admin' LIMIT 1), (SELECT id FROM "users" WHERE username='admin' LIMIT 1));


CREATE TABLE "user_application_permissions"(
	"user_id" INT4 NOT NULL,
	"application_permission_id" INT4 NOT NULL,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT "user_application_permissions_user_id_fk" FOREIGN KEY("user_id") REFERENCES "users"("id") ON DELETE CASCADE,
  CONSTRAINT "user_application_permissions_application_permission_id_fk" FOREIGN KEY("application_permission_id") REFERENCES "application_permissions"("id") ON DELETE CASCADE,
  PRIMARY KEY("user_id", "application_permission_id")
);

INSERT INTO "user_application_permissions" ("user_id", "application_permission_id")
  VALUES
    ((SELECT id FROM "users" WHERE username='admin' LIMIT 1), (SELECT id FROM "application_permissions" WHERE uri='admin' LIMIT 1));

