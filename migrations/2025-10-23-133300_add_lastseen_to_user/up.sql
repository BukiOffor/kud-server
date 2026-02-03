-- Your SQL goes here





ALTER TABLE "users" DROP COLUMN "full_name";
ALTER TABLE "users" ADD COLUMN "last_seen" TIMESTAMP;
ALTER TABLE "users" ADD COLUMN "last_name" TEXT NOT NULL;
ALTER TABLE "users" ADD COLUMN "first_name" TEXT NOT NULL;

