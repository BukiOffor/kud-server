-- Your SQL goes here
CREATE TABLE users_rosters (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    roster_id UUID NOT NULL,
    hall TEXT NOT NULL,
    year TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    CONSTRAINT fk_users_rosters_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,
    CONSTRAINT fk_users_rosters_roster
        FOREIGN KEY (roster_id)
        REFERENCES rosters(id)
        ON DELETE CASCADE
);

CREATE INDEX users_rosters_user_id_idx ON users_rosters(user_id);
CREATE INDEX users_rosters_roster_id_idx ON users_rosters(roster_id);
