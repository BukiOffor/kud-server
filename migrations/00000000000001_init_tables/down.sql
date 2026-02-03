-- =========================
-- USER ATTENDANCE
-- =========================
DROP INDEX IF EXISTS user_attendance_unique_idx;
DROP TABLE IF EXISTS user_attendance;

-- =========================
-- EVENTS
-- =========================
DROP INDEX IF EXISTS events_date_idx;
DROP TABLE IF EXISTS events;

-- =========================
-- USERS
-- =========================
DROP INDEX IF EXISTS users_email_idx;
DROP INDEX IF EXISTS users_reg_no_idx;
DROP TABLE IF EXISTS users;

