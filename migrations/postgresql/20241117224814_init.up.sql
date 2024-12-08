CREATE EXTENSION IF NOT EXISTS "pgcrypto";


CREATE TABLE "tenents" (
	"id" SERIAL PRIMARY KEY,
	"client_id" VARCHAR(36) NOT NULL,
  "issuer" TEXT NOT NULL,
  "audience" TEXT,
	"algorithm" TEXT NOT NULL DEFAULT 'HS256',
	"public_key" TEXT,
	"private_key" TEXT NOT NULL,
	"expires_in_seconds" BIGINT NOT NULL DEFAULT 86400,
	"refresh_expires_in_seconds" BIGINT NOT NULL DEFAULT 604800,
	"updated_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
	"created_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc')
);
CREATE UNIQUE INDEX "tenents_client_id_unique_idx" ON "tenents" ("client_id");

INSERT INTO "tenents"
  ("client_id", "issuer", "private_key")
  VALUES
	('6fcf0235-cb11-4160-9df8-b9114f8dcdae', 'Admin', encode(public.gen_random_bytes(255), 'base64'));


CREATE TABLE "tenent_oauth2_providers" (
	"id" SERIAL PRIMARY KEY,
	"tenent_id" BIGINT NOT NULL,
  "provider" TEXT NOT NULL,
  "active" SMALLINT NOT NULL DEFAULT 1,
  "client_id" TEXT NOT NULL,
  "client_secret" TEXT NOT NULL,
  "auth_url" TEXT NOT NULL,
  "token_url" TEXT NOT NULL,
  "callback_url" TEXT,
  "redirect_url" TEXT NOT NULL,
  "scope" TEXT NOT NULL,
	"updated_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
	"created_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
  FOREIGN KEY ("tenent_id") REFERENCES "tenents" ("id") ON DELETE CASCADE
);
CREATE UNIQUE INDEX "tenent_oauth2_providers_tenent_id_provider_unique_idx" ON "tenent_oauth2_providers" ("tenent_id", "provider");


CREATE TABLE "service_accounts" (
	"id" SERIAL PRIMARY KEY,
	"client_id" VARCHAR(36) NOT NULL,
  "encrypted_client_secret" TEXT NOT NULL,
  "name" TEXT NOT NULL,
  "active" SMALLINT NOT NULL DEFAULT 1,
	"updated_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
	"created_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc')
);
CREATE UNIQUE INDEX "service_accounts_client_id_unique_idx" ON "service_accounts" ("client_id");
CREATE UNIQUE INDEX "service_accounts_name_unique_idx" ON "service_accounts" ("name");


CREATE TABLE "users"(
	"id" SERIAL PRIMARY KEY,
  "username" TEXT NOT NULL,
  "active" SMALLINT NOT NULL DEFAULT 1,
	"updated_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
	"created_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc')
);
CREATE UNIQUE INDEX "users_username_unique_idx" ON "users" ("username");


CREATE TABLE "user_infos"(
	"user_id" BIGINT NOT NULL PRIMARY KEY,
	"name" TEXT,
	"given_name" TEXT,
	"family_name" TEXT,
	"middle_name" TEXT,
	"nickname" TEXT,
	"profile_picture" TEXT,
	"website" TEXT,
	"gender" TEXT,
	"birthdate" BIGINT,
	"zone_info" TEXT,
	"locale" TEXT,
	"address" TEXT,
	"updated_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
	"created_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
  CONSTRAINT "user_infos_user_id_fk" FOREIGN KEY("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);
CREATE UNIQUE INDEX "user_infos_user_id_unique_idx" ON "user_infos" ("user_id");


CREATE TABLE "user_passwords"(
	"id" SERIAL PRIMARY KEY,
	"user_id" BIGINT NOT NULL,
	"active" SMALLINT NOT NULL DEFAULT 1,
	"encrypted_password" TEXT NOT NULL,
	"updated_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
	"created_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
	CONSTRAINT "user_passwords_user_id_fk" FOREIGN KEY("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);


CREATE TABLE "user_totps"(
	"user_id" BIGINT NOT NULL PRIMARY KEY,
  "active" SMALLINT NOT NULL DEFAULT 1,
	"algorithm" TEXT NOT NULL DEFAULT 'SHA1',
  "digits" BIGINT NOT NULL DEFAULT 6,
  "step" BIGINT NOT NULL DEFAULT 30,
  "secret" TEXT NOT NULL,
	"updated_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
	"created_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
  CONSTRAINT "user_totps_user_id_fk" FOREIGN KEY("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);
CREATE UNIQUE INDEX "user_totps_user_id_unique_idx" ON "user_totps" ("user_id");


CREATE TABLE "user_emails" (
  "id" SERIAL PRIMARY KEY,
  "user_id" BIGINT NOT NULL,
  "email" TEXT NOT NULL,
  "verified" SMALLINT NOT NULL DEFAULT 0,
  "primary" SMALLINT NOT NULL DEFAULT 0,
	"updated_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
	"created_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
  CONSTRAINT "user_emails_user_id_fk" FOREIGN KEY("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);
CREATE UNIQUE INDEX "user_emails_email_unique_idx" ON "user_emails" ("email");


CREATE TABLE "user_phone_numbers" (
  "id" SERIAL PRIMARY KEY,
  "user_id" BIGINT NOT NULL,
  "phone_number" TEXT NOT NULL,
  "verified" SMALLINT NOT NULL DEFAULT 0,
  "primary" SMALLINT NOT NULL DEFAULT 0,
	"updated_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
	"created_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
  CONSTRAINT "user_phone_numbers_user_id_fk" FOREIGN KEY("user_id") REFERENCES "users"("id") ON DELETE CASCADE
);


CREATE TABLE "user_oauth2_providers" (
	"id" SERIAL PRIMARY KEY,
  "user_id" BIGINT NOT NULL,
  "tenent_oauth2_provider_id" BIGINT NOT NULL,
  "email" TEXT NOT NULL,
	"updated_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
	"created_at" BIGINT NOT NULL DEFAULT extract(epoch from now() at time zone 'utc'),
  CONSTRAINT "user_oauth2_providers_user_id_fk" FOREIGN KEY("user_id") REFERENCES "users"("id") ON DELETE CASCADE,
  CONSTRAINT "user_oauth2_providers_tenent_oauth2_provider_id_fk" FOREIGN KEY("tenent_oauth2_provider_id") REFERENCES "tenent_oauth2_providers"("id") ON DELETE CASCADE
);
CREATE UNIQUE INDEX "user_oauth2_providers_tenent_oauth2_provider_id_email_unique_idx" ON "user_oauth2_providers" ("tenent_oauth2_provider_id", "email");