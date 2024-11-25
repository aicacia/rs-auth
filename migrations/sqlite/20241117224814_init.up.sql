CREATE TABLE "tenents" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
	"client_id" BLOB NOT NULL,
  "issuer" TEXT NOT NULL,
  "audience" TEXT NOT NULL,
	"algorithm" VARCHAR(255) NOT NULL DEFAULT 'HS256',
	"public_key" TEXT,
	"private_key" TEXT NOT NULL,
	"expires_in_seconds" INTEGER NOT NULL DEFAULT 86400,
	"refresh_expires_in_seconds" INTEGER NOT NULL DEFAULT 604800,
	"updated_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
	"created_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now'))
);
CREATE UNIQUE INDEX "tenents_client_id_unique_idx" ON "tenents" ("client_id");

INSERT INTO "tenents"
  ("client_id", "issuer", "audience", "private_key")
  VALUES
	('6fcf0235-cb11-4160-9df8-b9114f8dcdae', 'Test', "http://localhost:3000", hex(randomblob(255)));


CREATE TABLE "service_accounts" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
	"client_id" BLOB NOT NULL,
  "encrypted_secret" TEXT NOT NULL,
  "name" VARCHAR(255) NOT NULL,
  "active" BOOLEAN NOT NULL DEFAULT TRUE,
  "updated_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now'))
);
CREATE UNIQUE INDEX "service_accounts_client_id_unique_idx" ON "service_accounts" ("client_id");
CREATE UNIQUE INDEX "service_accounts_name_unique_idx" ON "service_accounts" ("name");

INSERT INTO "service_accounts" 
	("name", "client_id", "encrypted_secret") 
	VALUES 
	("test", "dba9fb13-f2d0-498e-aaf2-65c435ffe797", "$argon2id$v=19$m=19456,t=2,p=1$ZU1WNU9Cc21rWFhOdkhpaw$pfGvO5zldmhSpS8kyx9PyzYMtzi6jpY9kkLyjmL+AD4");


CREATE TABLE "users" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "username" VARCHAR(255) NOT NULL,
  "active" BOOLEAN NOT NULL DEFAULT TRUE,
  "updated_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now'))
);
CREATE UNIQUE INDEX "users_username_unique_idx" ON "users" ("username");

INSERT INTO "users" ("username") 
	VALUES 
	("test");


CREATE TABLE "user_infos"(
	"user_id" INTEGER PRIMARY KEY,
	"name" VARCHAR(255),
	"given_name" VARCHAR(255),
	"family_name" VARCHAR(255),
	"middle_name" VARCHAR(255),
	"nickname" VARCHAR(255),
	"profile_picture" VARCHAR(255),
	"website" VARCHAR(255),
	"gender" VARCHAR(255),
	"birthdate" DATETIME,
	"zone_info" VARCHAR(255),
	"locale" VARCHAR(255),
	"address" VARCHAR(255),
  "updated_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);

INSERT INTO "user_infos" 
	("user_id")
	VALUES 
	((SELECT id FROM "users" WHERE username='test' LIMIT 1));


CREATE TABLE "user_passwords" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "active" BOOLEAN NOT NULL DEFAULT TRUE,
  "encrypted_password" TEXT NOT NULL,
  "updated_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);

INSERT INTO "user_passwords" 
	("user_id", "encrypted_password")
	VALUES 
	((SELECT id FROM "users" WHERE username='test' LIMIT 1), '$argon2id$v=19$m=19456,t=2,p=1$SmhvV0Y1WUZTU2YyS1MxVA$x0HaVNrXTCZSJa7zJzT3v59PQedgZquZlWYnp848cpE');


CREATE TABLE "user_emails" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "email" VARCHAR(255) NOT NULL,
  "verified" BOOLEAN NOT NULL DEFAULT FALSE,
  "primary" BOOLEAN NOT NULL DEFAULT FALSE,
  "updated_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);


CREATE TABLE "user_phone_numbers" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "phone_number" VARCHAR(255) NOT NULL,
  "verified" BOOLEAN NOT NULL DEFAULT FALSE,
  "primary" BOOLEAN NOT NULL DEFAULT FALSE,
  "updated_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);


CREATE TABLE "user_oauth2_providers" (
  "id" INTEGER PRIMARY KEY AUTOINCREMENT,
  "user_id" INTEGER NOT NULL,
  "provider" VARCHAR(255) NOT NULL,
  "email" VARCHAR(255) NOT NULL,
  "updated_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  "created_at" DATETIME NOT NULL DEFAULT (strftime('%s', 'now')),
  FOREIGN KEY ("user_id") REFERENCES "users" ("id") ON DELETE CASCADE
);
CREATE UNIQUE INDEX "user_oauth2_providers_user_id_provider_email_unique_idx" ON "user_oauth2_providers" ("user_id", "provider", "email");