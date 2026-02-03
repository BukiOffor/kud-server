-- This file should undo anything in `up.sql`
ALTER TABLE "huddle" DROP COLUMN "time_of_actual_end";
ALTER TABLE "huddle" DROP COLUMN "scheduled_start";
ALTER TABLE "huddle" DROP COLUMN "scheduled_end";
ALTER TABLE "huddle" DROP COLUMN "time_of_actual_start";
ALTER TABLE "huddle" ADD COLUMN "scheduled_at" TIMESTAMP NOT NULL;






