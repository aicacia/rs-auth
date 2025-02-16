CREATE TABLE "applications" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "name" TEXT NOT NULL,
	"updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;
CREATE UNIQUE INDEX "applications_id_unique_idx" ON "applications" ("id");


INSERT INTO "applications"
  ("name")
  VALUES
	('Admin');


CREATE TABLE "tenants" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "application_id" INTEGER NOT NULL,
	"client_id" TEXT NOT NULL,
  "issuer" TEXT NOT NULL,
  "audience" TEXT,
	"algorithm" TEXT NOT NULL DEFAULT 'HS256',
	"public_key" TEXT,
	"private_key" TEXT NOT NULL,
	"expires_in_seconds" INTEGER NOT NULL DEFAULT 86400,
	"refresh_expires_in_seconds" INTEGER NOT NULL DEFAULT 604800,
	"updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("application_id") REFERENCES "applications" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "tenants_id_unique_idx" ON "tenants" ("id");
CREATE UNIQUE INDEX "tenants_client_id_unique_idx" ON "tenants" ("client_id");
CREATE INDEX "tenants_application_id_idx" ON "tenants" ("application_id");

INSERT INTO "tenants"
  ("application_id", "client_id", "issuer", "audience", "private_key")
  VALUES
	(1, '6fcf0235-cb11-4160-9df8-b9114f8dcdae', 'Admin', null, hex(randomblob(255)));


CREATE TABLE "tenant_oauth2_providers" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "tenant_id" INTEGER NOT NULL,
  "provider" TEXT NOT NULL,
  "active" INTEGER NOT NULL DEFAULT 1,
  "client_id" TEXT NOT NULL,
  "client_secret" TEXT NOT NULL,
  "auth_url" TEXT NOT NULL,
  "token_url" TEXT NOT NULL,
  "callback_url" TEXT,
  "redirect_url" TEXT NOT NULL,
  "scope" TEXT NOT NULL,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("tenant_id") REFERENCES "tenants" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "tenant_oauth2_providers_id_unique_idx" ON "tenant_oauth2_providers" ("id");
CREATE UNIQUE INDEX "tenant_oauth2_providers_tenant_id_provider_unique_idx" ON "tenant_oauth2_providers" ("tenant_id", "provider");
CREATE INDEX "tenant_oauth2_providers_tenant_id_idx" ON "tenant_oauth2_providers" ("tenant_id");


CREATE TABLE "service_accounts" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "application_id" INTEGER NOT NULL,
	"client_id" TEXT NOT NULL,
  "encrypted_client_secret" TEXT NOT NULL,
  "name" TEXT NOT NULL,
  "active" INTEGER NOT NULL DEFAULT 1,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;
CREATE UNIQUE INDEX "service_accounts_id_unique_idx" ON "service_accounts" ("id");
CREATE UNIQUE INDEX "service_accounts_client_id_unique_idx" ON "service_accounts" ("client_id");
CREATE INDEX "service_accounts_application_id_idx" ON "service_accounts" ("application_id");


CREATE TABLE "users" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "application_id" INTEGER NOT NULL,
  "username" TEXT NOT NULL,
  "active" INTEGER NOT NULL DEFAULT 1,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("application_id") REFERENCES "applications" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "users_id_unique_idx" ON "users" ("id");
CREATE UNIQUE INDEX "users_application_id_username_unique_idx" ON "users" ("application_id", "username");
CREATE INDEX "users_application_id_idx" ON "users" ("application_id");


CREATE TABLE "user_configs" (
	"user_id" INTEGER NOT NULL PRIMARY KEY,
  "mfa_type" TEXT,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "user_configs_user_id_unique_idx" ON "user_configs" ("user_id");


CREATE TABLE "user_infos"(
	"user_id" INTEGER NOT NULL PRIMARY KEY,
	"name" TEXT,
	"given_name" TEXT,
	"family_name" TEXT,
	"middle_name" TEXT,
	"nickname" TEXT,
	"profile_picture" TEXT,
	"website" TEXT,
	"gender" TEXT,
	"birthdate" INTEGER,
	"zone_info" TEXT,
	"locale" TEXT,
	"address" TEXT,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "user_infos_user_id_unique_idx" ON "user_infos" ("user_id");


CREATE TABLE "user_passwords" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "active" INTEGER NOT NULL DEFAULT 1,
  "encrypted_password" TEXT NOT NULL,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "user_passwords_id_unique_idx" ON "user_passwords" ("id");
CREATE INDEX "user_passwords_user_id_idx" ON "user_passwords" ("user_id");


CREATE TABLE "user_totps"(
	"user_id" INTEGER NOT NULL PRIMARY KEY,
  "active" INTEGER NOT NULL DEFAULT 1,
	"algorithm" TEXT NOT NULL DEFAULT 'SHA1',
  "digits" INTEGER NOT NULL DEFAULT 6,
  "step" INTEGER NOT NULL DEFAULT 30,
  "secret" TEXT NOT NULL,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "user_totps_user_id_unique_idx" ON "user_totps" ("user_id");


CREATE TABLE "user_emails" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "email" TEXT NOT NULL,
  "verified" INTEGER NOT NULL DEFAULT 0,
  "primary" INTEGER NOT NULL DEFAULT 0,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "user_emails_id_unique_idx" ON "user_emails" ("id");
CREATE UNIQUE INDEX "user_emails_email_unique_idx" ON "user_emails" ("email");
CREATE INDEX "user_emails_user_id_idx" ON "user_emails" ("user_id");


CREATE TABLE "user_phone_numbers" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "phone_number" TEXT NOT NULL,
  "verified" INTEGER NOT NULL DEFAULT 0,
  "primary" INTEGER NOT NULL DEFAULT 0,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "user_phone_numbers_id_unique_idx" ON "user_phone_numbers" ("id");
CREATE UNIQUE INDEX "user_phone_numbers_phone_number_unique_idx" ON "user_phone_numbers" ("phone_number");
CREATE INDEX "user_phone_numbers_user_id_idx" ON "user_phone_numbers" ("user_id");


CREATE TABLE "user_oauth2_providers" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "tenant_oauth2_provider_id" INTEGER NOT NULL,
  "email" TEXT NOT NULL,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE,
  FOREIGN KEY ("tenant_oauth2_provider_id") REFERENCES "tenant_oauth2_providers" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "user_oauth2_providers_id_unique_idx" ON "user_oauth2_providers" ("id");
CREATE UNIQUE INDEX "user_oauth2_providers_tenant_oauth2_provider_id_email_unique_idx" ON "user_oauth2_providers" ("tenant_oauth2_provider_id", "email");
CREATE INDEX "user_oauth2_providers_user_id_idx" ON "user_oauth2_providers" ("user_id");
CREATE INDEX "user_oauth2_providers_tenant_oauth2_provider_id_idx" ON "user_oauth2_providers" ("tenant_oauth2_provider_id");