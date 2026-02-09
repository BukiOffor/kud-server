-- This file should undo anything in `up.sql`
ALTER TABLE users_rosters DROP CONSTRAINT users_rosters_user_roster_unique;
