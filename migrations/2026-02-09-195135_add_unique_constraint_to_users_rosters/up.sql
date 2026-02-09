-- Your SQL goes here
DELETE FROM users_rosters a USING users_rosters b
WHERE a.id < b.id
  AND a.user_id = b.user_id
  AND a.roster_id = b.roster_id;

ALTER TABLE users_rosters ADD CONSTRAINT users_rosters_user_roster_unique UNIQUE (user_id, roster_id);
