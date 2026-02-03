// @generated automatically by Diesel CLI.

diesel::table! {
    events (id) {
        id -> Uuid,
        title -> Text,
        description -> Text,
        date -> Date,
        time -> Time,
        grace_period_in_minutes -> Int4,
        attendance_type -> Text,
        location -> Text,
        created_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    user_attendance (id) {
        id -> Uuid,
        user_id -> Uuid,
        date -> Date,
        time_in -> Timestamp,
        time_out -> Nullable<Timestamp>,
        marked_by -> Nullable<Uuid>,
        event_id -> Nullable<Uuid>,
        attendance_type -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        username -> Nullable<Text>,
        reg_no -> Text,
        email -> Text,
        password_hash -> Text,
        dob -> Nullable<Timestamp>,
        avatar_url -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        last_seen -> Nullable<Timestamp>,
        last_name -> Text,
        first_name -> Text,
        year_joined -> Text,
        current_roster_hall -> Nullable<Text>,
        current_roster_allocation -> Nullable<Text>,
        role -> Text,
        device_id -> Nullable<Text>,
        is_active -> Bool,
        gender -> Nullable<Text>,
        address -> Nullable<Text>,
        city -> Nullable<Text>,
        state -> Nullable<Text>,
        country -> Nullable<Text>,
        phone -> Nullable<Text>,
    }
}

diesel::joinable!(events -> users (created_by));
diesel::joinable!(user_attendance -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    events,
    user_attendance,
    users,
);
