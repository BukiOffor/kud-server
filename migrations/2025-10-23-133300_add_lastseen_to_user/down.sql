-- This file should undo anything in `up.sql`





ALTER TABLE "users" DROP COLUMN "last_seen";
ALTER TABLE "users" DROP COLUMN "last_name";
ALTER TABLE "users" DROP COLUMN "first_name";
ALTER TABLE "users" ADD COLUMN "full_name" TEXT NOT NULL;

