CREATE TABLE "user" (
	"u_id" uuid PRIMARY KEY,
	"u_name" varchar(128) NOT NULL,
	"u_pri_email" varchar(128) NULL DEFAULT NULL
);
CREATE UNIQUE INDEX "u_idx_id" ON "user" ("u_id");
CREATE UNIQUE INDEX "u_idx_name" ON "user" ("u_name");

CREATE TABLE "user_group" (
	"ug_user" uuid NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE,
	"ug_group" varchar(32) NOT NULL,
	"ug_expiry" timestamp NULL DEFAULT NULL,
	PRIMARY KEY("ug_user", "ug_group")
);
CREATE INDEX "ug_idx_user" ON "user_group" ("ug_user");
CREATE INDEX "ug_idx_group" ON "user_group" ("ug_group");

CREATE TABLE "user_email" (
	"uemail_user" uuid NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE,
	"uemail_email" varchar(128) NOT NULL,
	PRIMARY KEY("uemail_user", "uemail_email")
);
CREATE INDEX "uemail_idx_user" ON "user_email" ("uemail_user");
CREATE INDEX "uemail_idx_email" ON "user_email" ("uemail_email");

CREATE TABLE "user_alias" (
	"ualias_alias" varchar(128) PRIMARY KEY,
	"ualias_user" uuid NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE
);
CREATE UNIQUE INDEX "ualias_idx_alias" ON "user_alias" ("ualias_alias");
CREATE INDEX "ualias_idx_user" ON "user_alias" ("ualias_user");

CREATE TABLE "block" (
	"block_user" uuid NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE,
	"block_type" varchar(32) NOT NULL,
	"block_expiry" timestamp NULL DEFAULT NULL,
	"block_reason" text NULL DEFAULT NULL,
	PRIMARY KEY("block_user", "block_type")
);
CREATE INDEX "block_idx_user" ON "block" ("block_user");
CREATE INDEX "block_idx_type" ON "block" ("block_type");

CREATE TABLE "logging" (
	"log_id" uuid PRIMARY KEY,
	"log_user" uuid NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE,
	"log_type" varchar(16) NOT NULL,
	"log_action" varchar(16) NOT NULL,
	"log_target" uuid NOT NULL REFERENCES "user"("u_id") ON DELETE CASCADE,
	"log_params" jsonb NOT NULL
);
CREATE UNIQUE INDEX "log_idx_id" ON "logging" ("log_id");
CREATE INDEX "log_idx_user" ON "logging" ("log_user") INCLUDE ("log_id", "log_type", "log_target");
CREATE INDEX "log_idx_type" ON "logging" ("log_type") INCLUDE ("log_id");
CREATE INDEX "log_idx_target" ON "logging" ("log_target") INCLUDE ("log_id", "log_type");
