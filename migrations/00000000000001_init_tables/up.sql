-- =========================
-- USERS
-- =========================
CREATE TABLE users (
    id UUID PRIMARY KEY,

    username TEXT,
    reg_no TEXT NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT NOT NULL,

    dob TIMESTAMP,
    avatar_url TEXT,

    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),
    last_seen TIMESTAMP,

    last_name TEXT NOT NULL,
    first_name TEXT NOT NULL,

    year_joined TEXT NOT NULL,

    current_roster_hall TEXT,
    current_roster_allocation TEXT,

    role TEXT NOT NULL,
    device_id TEXT,

    is_active BOOLEAN NOT NULL DEFAULT true,

    gender TEXT,
    address TEXT,
    city TEXT,
    state TEXT,
    country TEXT,
    phone TEXT
);

CREATE UNIQUE INDEX users_email_idx ON users(email);
CREATE UNIQUE INDEX users_reg_no_idx ON users(reg_no);

-- =========================
-- EVENTS
-- =========================
CREATE TABLE events (
    id UUID PRIMARY KEY,

    title TEXT NOT NULL,
    description TEXT NOT NULL,

    date DATE NOT NULL,
    time TIME NOT NULL,

    grace_period_in_minutes INTEGER NOT NULL DEFAULT 0,
    attendance_type TEXT NOT NULL,
    location TEXT NOT NULL,

    created_by UUID NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_events_created_by
        FOREIGN KEY (created_by)
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE INDEX events_date_idx ON events(date);

-- =========================
-- USER ATTENDANCE
-- =========================
CREATE TABLE user_attendance (
    id UUID PRIMARY KEY,

    user_id UUID NOT NULL,
    date DATE NOT NULL,

    time_in TIMESTAMP NOT NULL,
    time_out TIMESTAMP,

    marked_by UUID,
    event_id UUID,

    attendance_type TEXT NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT now(),
    updated_at TIMESTAMP NOT NULL DEFAULT now(),

    CONSTRAINT fk_attendance_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_attendance_marker
        FOREIGN KEY (marked_by)
        REFERENCES users(id)
        ON DELETE SET NULL,

    CONSTRAINT fk_attendance_event
        FOREIGN KEY (event_id)
        REFERENCES events(id)
        ON DELETE SET NULL
);

CREATE UNIQUE INDEX user_attendance_unique_idx
ON user_attendance (user_id, date, attendance_type);
