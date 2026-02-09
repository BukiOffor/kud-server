-- Your SQL goes here
CREATE TABLE rosters (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT false,
    start_date DATE NOT NULL,
    num_for_hall_one INTEGER NOT NULL,
    num_for_main_hall INTEGER NOT NULL,
    num_for_gallery INTEGER NOT NULL,
    num_for_basement INTEGER NOT NULL,
    num_for_outside INTEGER NOT NULL,
    end_date DATE NOT NULL,
    year TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT now()
);

CREATE TABLE activity_logs (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    activity_type TEXT NOT NULL,
    target_id UUID,
    target_type TEXT,
    details JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP NOT NULL DEFAULT now(),
    CONSTRAINT fk_activity_logs_user
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE
);

CREATE INDEX activity_logs_user_id_idx ON activity_logs(user_id);
