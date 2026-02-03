-- Your SQL goes here
ALTER TABLE "huddle" DROP COLUMN "scheduled_at";
ALTER TABLE "huddle" ADD COLUMN "time_of_actual_end" TIMESTAMP;
ALTER TABLE "huddle" ADD COLUMN "scheduled_start" TIMESTAMP NOT NULL;
ALTER TABLE "huddle" ADD COLUMN "scheduled_end" TIMESTAMP NOT NULL;
ALTER TABLE "huddle" ADD COLUMN "time_of_actual_start" TIMESTAMP;






