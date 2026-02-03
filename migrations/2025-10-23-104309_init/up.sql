-- Your SQL goes here
CREATE TABLE "huddle"(
	"id" UUID NOT NULL PRIMARY KEY,
	"title" TEXT NOT NULL UNIQUE,
	"description" TEXT,
	"scheduled_at" TIMESTAMP NOT NULL,
	"is_public" BOOL NOT NULL,
	"status" TEXT NOT NULL,
	"host_id" UUID NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "huddle_interest"(
	"user_id" UUID NOT NULL,
	"huddle_id" UUID NOT NULL,
	PRIMARY KEY("user_id", "huddle_id")
);

CREATE TABLE "huddle_participants"(
	"user_id" UUID NOT NULL,
	"huddle_id" UUID NOT NULL,
	"joined_at" TIMESTAMP NOT NULL,
	PRIMARY KEY("user_id", "huddle_id")
);

CREATE TABLE "huddle_recordings"(
	"huddle_id" UUID NOT NULL PRIMARY KEY,
	"session_id" UUID NOT NULL,
	"file_url" TEXT NOT NULL,
	"duration_seconds" INT4,
	"created_at" TIMESTAMP NOT NULL
);

CREATE TABLE "users"(
	"id" UUID NOT NULL PRIMARY KEY,
	"full_name" TEXT NOT NULL,
	"username" TEXT NOT NULL UNIQUE,
	"email" TEXT NOT NULL UNIQUE,
	"password_hash" TEXT NOT NULL,
	"bio" TEXT,
	"avatar_url" TEXT,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

CREATE TABLE "saved_huddle"(
	"user_id" UUID NOT NULL,
	"huddle_id" UUID NOT NULL,
	"saved_at" TIMESTAMP NOT NULL,
	PRIMARY KEY("user_id", "huddle_id")
);

