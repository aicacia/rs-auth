CREATE EXTENSION IF NOT EXISTS "pgcrypto";


CREATE FUNCTION "trigger_set_timestamp"()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = CURRENT_TIMESTAMP;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;


CREATE TABLE "config" (
	"name" TEXT NOT NULL PRIMARY KEY,
	"value" JSONB NOT NULL DEFAULT 'null',
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER "config_set_timestamp" BEFORE UPDATE ON "config" FOR EACH ROW EXECUTE PROCEDURE "trigger_set_timestamp"();

INSERT INTO "config" ("name", "value") VALUES
  ('server.address', '"0.0.0.0"'),
  ('server.port', '8080'),
  ('server.uri', '"http://localhost:8080"'),
  ('log_level', '"debug"'),
  ('allow_public_signup', 'false');


CREATE FUNCTION config_notify() RETURNS trigger AS $$
DECLARE
  "name" TEXT;
  "value" JSONB;
BEGIN
  IF TG_OP = 'INSERT' OR TG_OP = 'UPDATE' THEN
  "name" = NEW."name";
  ELSE
  "name" = OLD."name";
  END IF;
  IF TG_OP != 'UPDATE' OR NEW."value" != OLD."value" THEN
  PERFORM pg_notify('config_channel', json_build_object('table', TG_TABLE_NAME, 'name', "name", 'value', NEW."value", 'action_type', TG_OP)::text);
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER "config_notify_update" AFTER UPDATE ON "config" FOR EACH ROW EXECUTE PROCEDURE config_notify();
CREATE TRIGGER "config_notify_insert" AFTER INSERT ON "config" FOR EACH ROW EXECUTE PROCEDURE config_notify();
CREATE TRIGGER "config_notify_delete" AFTER DELETE ON "config" FOR EACH ROW EXECUTE PROCEDURE config_notify();


CREATE TABLE "applications" (
	"id" SERIAL PRIMARY KEY,
  "name" VARCHAR(255) NOT NULL,
  "uri" VARCHAR(255) NOT NULL,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER "applications_set_timestamp" BEFORE UPDATE ON "applications" FOR EACH ROW EXECUTE PROCEDURE "trigger_set_timestamp"();

INSERT INTO "applications" ("name", "uri") VALUES
  ('Admin', 'admin');


CREATE TABLE "application_configs" (
  "application_id" INT4 NOT NULL,
	"name" TEXT NOT NULL,
	"value" JSONB NOT NULL DEFAULT 'null',
	"created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
	"updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
  CONSTRAINT "application_configs_application_id_fk" FOREIGN KEY("application_id") REFERENCES "applications"("id") ON DELETE CASCADE,
  PRIMARY KEY("application_id", "name")
);
CREATE TRIGGER "application_configs_set_timestamp" BEFORE UPDATE ON "application_configs" FOR EACH ROW EXECUTE PROCEDURE "trigger_set_timestamp"();

INSERT INTO "application_configs" ("application_id", "name", "value") VALUES
  (1, 'jwt.secret', (CONCAT('"', translate(encode(gen_random_bytes(255), 'base64'), E'+/=\n', '-_'), '"'))::JSONB),
  (1, 'jwt.expires_in_seconds', '86400'),
  (1, 'default_role', '2'),
  (1, 'uri', '"http://localhost:8080"'),
  (1, 'mail.support.email', '"support@localhost.com"'),
  (1, 'mail.support.name', '"Support"');


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
	"email" VARCHAR(320) NOT NULL,
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
  VALUES (1, 1);

