CREATE TABLE "tenents" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"client_id" TEXT NOT NULL,
  "issuer" TEXT NOT NULL,
  "audience" TEXT NOT NULL,
	"algorithm" TEXT NOT NULL DEFAULT 'HS256',
	"public_key" TEXT,
	"private_key" TEXT NOT NULL,
	"expires_in_seconds" INTEGER NOT NULL DEFAULT 86400,
	"refresh_expires_in_seconds" INTEGER NOT NULL DEFAULT 604800,
	"updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
	"created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;
CREATE UNIQUE INDEX "tenents_id_unique_idx" ON "tenents" ("id");
CREATE UNIQUE INDEX "tenents_client_id_unique_idx" ON "tenents" ("client_id");

INSERT INTO "tenents"
  ("client_id", "issuer", "audience", "private_key")
  VALUES
	('6fcf0235-cb11-4160-9df8-b9114f8dcdae', 'Test', "http://localhost:3000", hex(randomblob(255)));


CREATE TABLE "tenent_oauth2_providers" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "tenent_id" INTEGER NOT NULL,
  "provider" TEXT NOT NULL,
  "active" INTEGER NOT NULL DEFAULT TRUE,
  "client_id" TEXT NOT NULL,
  "client_secret" TEXT NOT NULL,
  "auth_url" TEXT NOT NULL,
  "token_url" TEXT NOT NULL,
  "callback_url" TEXT,
  "redirect_url" TEXT NOT NULL,
  "scope" TEXT NOT NULL,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("tenent_id") REFERENCES "tenents" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "tenent_oauth2_providers_id_unique_idx" ON "tenent_oauth2_providers" ("id");
CREATE UNIQUE INDEX "tenent_oauth2_providers_tenent_id_provider_unique_idx" ON "tenent_oauth2_providers" ("tenent_id", "provider");
CREATE INDEX "tenent_oauth2_providers_tenent_id_idx" ON "tenent_oauth2_providers" ("tenent_id");


CREATE TABLE "service_accounts" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	"client_id" TEXT NOT NULL,
  "encrypted_secret" TEXT NOT NULL,
  "name" TEXT NOT NULL,
  "active" INTEGER NOT NULL DEFAULT TRUE,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;
CREATE UNIQUE INDEX "service_accounts_id_unique_idx" ON "service_accounts" ("id");
CREATE UNIQUE INDEX "service_accounts_client_id_unique_idx" ON "service_accounts" ("client_id");
CREATE UNIQUE INDEX "service_accounts_name_unique_idx" ON "service_accounts" ("name");

INSERT INTO "service_accounts" 
	("name", "client_id", "encrypted_secret") 
	VALUES 
	('test', 'dba9fb13-f2d0-498e-aaf2-65c435ffe797', '$argon2id$v=19$m=19456,t=2,p=1$ZU1WNU9Cc21rWFhOdkhpaw$pfGvO5zldmhSpS8kyx9PyzYMtzi6jpY9kkLyjmL+AD4');


CREATE TABLE "users" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "username" TEXT NOT NULL,
  "active" INTEGER NOT NULL DEFAULT TRUE,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
) STRICT;
CREATE UNIQUE INDEX "users_id_unique_idx" ON "users" ("id");
CREATE UNIQUE INDEX "users_username_unique_idx" ON "users" ("username");

INSERT INTO "users" ("username") 
	VALUES 
	("test");


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

INSERT INTO "user_infos" 
	("user_id")
	VALUES 
	((SELECT id FROM "users" WHERE username='test' LIMIT 1));


CREATE TABLE "user_passwords" (
  "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "active" INTEGER NOT NULL DEFAULT TRUE,
  "encrypted_password" TEXT NOT NULL,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "user_passwords_id_unique_idx" ON "user_passwords" ("id");
CREATE INDEX "user_passwords_user_id_idx" ON "user_passwords" ("user_id");

INSERT INTO "user_passwords" 
	("user_id", "encrypted_password")
	VALUES 
	((SELECT id FROM "users" WHERE username='test' LIMIT 1), '$argon2id$v=19$m=19456,t=2,p=1$SmhvV0Y1WUZTU2YyS1MxVA$x0HaVNrXTCZSJa7zJzT3v59PQedgZquZlWYnp848cpE');


CREATE TABLE "user_totps"(
	"user_id" INTEGER NOT NULL PRIMARY KEY,
  "active" INTEGER NOT NULL DEFAULT TRUE,
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
  "verified" INTEGER NOT NULL DEFAULT FALSE,
  "primary" INTEGER NOT NULL DEFAULT FALSE,
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
  "verified" INTEGER NOT NULL DEFAULT FALSE,
  "primary" INTEGER NOT NULL DEFAULT FALSE,
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
  "tenent_oauth2_provider_id" INTEGER NOT NULL,
  "email" TEXT NOT NULL,
  "updated_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE,
  FOREIGN KEY ("tenent_oauth2_provider_id") REFERENCES "tenent_oauth2_providers" ("id") ON DELETE CASCADE
) STRICT;
CREATE UNIQUE INDEX "user_oauth2_providers_id_unique_idx" ON "user_oauth2_providers" ("id");
CREATE UNIQUE INDEX "user_oauth2_providers_tenent_oauth2_provider_id_email_unique_idx" ON "user_oauth2_providers" ("tenent_oauth2_provider_id", "email");
CREATE INDEX "user_oauth2_providers_user_id_idx" ON "user_oauth2_providers" ("user_id");
CREATE INDEX "user_oauth2_providers_tenent_oauth2_provider_id_idx" ON "user_oauth2_providers" ("tenent_oauth2_provider_id");