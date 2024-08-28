CREATE TABLE "user" (
	"u_id" bigserial PRIMARY KEY,
	"u_name" varchar(128) NOT NULL,
	"u_pri_email" varchar(128) DEFAULT null
);
COMMENT ON TABLE "user" IS 'Users';
COMMENT ON COLUMN "user"."u_id" IS 'User identifier';
COMMENT ON COLUMN "user"."u_name" IS 'Primary name';
COMMENT ON COLUMN "user"."u_pri_email" IS 'Primary email address';
CREATE UNIQUE INDEX "u_id" ON "user" ("u_id");
CREATE UNIQUE INDEX "u_name" ON "user" ("u_name");

CREATE TABLE "user_group" (
	"ug_user" bigint NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE,
	"ug_group" varchar(32) NOT NULL,
	"ug_expiry" timestamp DEFAULT null,
	PRIMARY KEY("ug_user", "ug_group")
);
COMMENT ON TABLE "user_group" IS 'User-group memberships';
COMMENT ON COLUMN "user_group"."ug_user" IS 'User';
COMMENT ON COLUMN "user_group"."ug_group" IS 'Group';
COMMENT ON COLUMN "user_group"."ug_expiry" IS 'Membership expiry time';
CREATE INDEX "ug_user" ON "user_group" ("ug_user", "ug_group");
CREATE INDEX "ug_group" ON "user_group" ("ug_group");
CREATE INDEX "ug_expiry" ON "user_group" ("ug_expiry");

CREATE TABLE "user_email" (
	"uemail_user" bigint NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE,
	"uemail_email" varchar(64) NOT NULL,
	PRIMARY KEY("uemail_user", "uemail_email")
);
COMMENT ON TABLE "user_email" IS 'Email addresses linked to users';
COMMENT ON COLUMN "user_email"."uemail_user" IS 'User';
COMMENT ON COLUMN "user_email"."uemail_email" IS 'Email address';
CREATE INDEX "uemail_user" ON "user_email" ("uemail_user");
CREATE INDEX "uemail_email" ON "user_email" ("uemail_email");

CREATE TABLE "user_alias" (
	"ualias_alias" varchar(64) PRIMARY KEY,
	"ualias_user" bigint NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE
);
COMMENT ON TABLE "user_alias" IS 'User aliases';
COMMENT ON COLUMN "user_alias"."ualias_alias" IS 'Alias';
COMMENT ON COLUMN "user_alias"."ualias_user" IS 'User';
CREATE UNIQUE INDEX "ualias_alias" ON "user_alias" ("ualias_alias");
CREATE INDEX "ualias_user" ON "user_alias" ("ualias_user");

CREATE TABLE "block" (
	"block_user" bigint NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE,
	"block_type" varchar(32) NOT NULL,
	"block_expiry" timestamp DEFAULT null,
	"block_reason" text DEFAULT null,
	PRIMARY KEY("block_user", "block_type")
);
COMMENT ON TABLE "block" IS 'User blocks';
COMMENT ON COLUMN "block"."block_user" IS 'User';
COMMENT ON COLUMN "block"."block_type" IS 'Type';
COMMENT ON COLUMN "block"."block_expiry" IS 'Block expiry time';
COMMENT ON COLUMN "block"."block_reason" IS 'Block reason';
CREATE INDEX "block_user" ON "block" ("block_user", "block_type");
CREATE INDEX "block_type" ON "block" ("block_type");
CREATE INDEX "block_expiry" ON "block" ("block_expiry");

CREATE TABLE "logging" (
	"log_id" uuid PRIMARY KEY,
	"log_user" bigint NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE,
	"log_type" varchar(16) NOT NULL,
	"log_action" varchar(16) NOT NULL,
	"log_target_user" bigint DEFAULT null REFERENCES "user"("u_id") ON DELETE CASCADE,
	"log_target_secret" uuid DEFAULT null,
	"log_params" jsonb NOT NULL
);
COMMENT ON TABLE "logging" IS 'Log events';
COMMENT ON COLUMN "logging"."log_id" IS 'Log timestamp';
COMMENT ON COLUMN "logging"."log_user" IS 'Event performer';
COMMENT ON COLUMN "logging"."log_type" IS 'Log event type';
COMMENT ON COLUMN "logging"."log_action" IS 'Log event action';
COMMENT ON COLUMN "logging"."log_target_user" IS 'Target user';
COMMENT ON COLUMN "logging"."log_target_secret" IS 'Target secret';
COMMENT ON COLUMN "logging"."log_params" IS 'Event params';
CREATE UNIQUE INDEX "log_id" ON "logging" ("log_id");
CREATE INDEX "log_user" ON "logging" ("log_user", "log_type", "log_id") INCLUDE ("log_target_user", "log_target_secret");
CREATE INDEX "log_type" ON "logging" ("log_type", "log_id");
CREATE INDEX "log_target_user" ON "logging" ("log_target_user", "log_id") INCLUDE ("log_type");
CREATE INDEX "log_target_secret" ON "logging" ("log_target_secret", "log_id") INCLUDE ("log_type");

CREATE TABLE "password" (
	"passwd_id" uuid PRIMARY KEY,
	"passwd_user" bigint NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE,
	"passwd_password" bytea NOT NULL
);
COMMENT ON TABLE "password" IS 'Passwords';
COMMENT ON COLUMN "password"."passwd_id" IS 'Password ID';
COMMENT ON COLUMN "password"."passwd_user" IS 'User ID';
COMMENT ON COLUMN "password"."passwd_password" IS 'Password secret';
CREATE UNIQUE INDEX "passwd_id" ON "password" ("passwd_id");
CREATE INDEX "passwd_user" ON "password" ("passwd_user");

CREATE TABLE "user_tokens" (
	"utoken_id" uuid PRIMARY KEY,
	"utoken_token" varchar(64) UNIQUE NOT NULL,
	"utoken_user" bigint NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE,
	"utoken_label" varchar(128) NOT NULL DEFAULT '',
	"utoken_expiry" timestamp DEFAULT null,
	"utoken_last_used" timestamp DEFAULT null
);
COMMENT ON TABLE "user_tokens" IS 'User access tokens';
COMMENT ON COLUMN "user_tokens"."utoken_id" IS 'Access token ID';
COMMENT ON COLUMN "user_tokens"."utoken_token" IS 'Access token';
COMMENT ON COLUMN "user_tokens"."utoken_user" IS 'User ID';
COMMENT ON COLUMN "user_tokens"."utoken_label" IS 'Token label';
COMMENT ON COLUMN "user_tokens"."utoken_expiry" IS 'Token expiration time';
COMMENT ON COLUMN "user_tokens"."utoken_last_used" IS 'Token last used time';
CREATE UNIQUE INDEX "utoken_id" ON "user_tokens" ("utoken_id");
CREATE UNIQUE INDEX "utoken_token" ON "user_tokens" ("utoken_token");
CREATE UNIQUE INDEX "utoken_user" ON "user_tokens" ("utoken_user");
