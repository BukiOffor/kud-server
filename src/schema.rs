// @generated automatically by Diesel CLI.

diesel::table! {
    activity_logs (id) {
        id -> Uuid,
        user_id -> Uuid,
        activity_type -> Text,
        target_id -> Nullable<Uuid>,
        target_type -> Nullable<Text>,
        details -> Jsonb,
        created_at -> Timestamp,
    }
}

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
    rosters (id) {
        id -> Uuid,
        name -> Text,
        is_active -> Bool,
        start_date -> Date,
        num_for_hall_one -> Int4,
        num_for_main_hall -> Int4,
        num_for_gallery -> Int4,
        num_for_basement -> Int4,
        num_for_outside -> Int4,
        end_date -> Date,
        year -> Text,
        created_at -> Timestamp,
        num_male_for_hall_one -> Nullable<Int4>,
        num_female_for_hall_one -> Nullable<Int4>,
        num_male_for_main_hall -> Nullable<Int4>,
        num_female_for_main_hall -> Nullable<Int4>,
        num_male_for_gallery -> Nullable<Int4>,
        num_female_for_gallery -> Nullable<Int4>,
        num_male_for_basement -> Nullable<Int4>,
        num_female_for_basement -> Nullable<Int4>,
        num_male_for_outside -> Nullable<Int4>,
        num_female_for_outside -> Nullable<Int4>,
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
        week_day -> Text,
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
        hall_derivation -> Int4,
    }
}

diesel::table! {
    users_rosters (id) {
        id -> Uuid,
        user_id -> Uuid,
        roster_id -> Uuid,
        hall -> Text,
        year -> Text,
        created_at -> Timestamp,
    }
}

diesel::joinable!(activity_logs -> users (user_id));
diesel::joinable!(events -> users (created_by));
diesel::joinable!(user_attendance -> events (event_id));
diesel::joinable!(user_attendance -> users (user_id));
diesel::joinable!(users_rosters -> rosters (roster_id));
diesel::joinable!(users_rosters -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    activity_logs,
    events,
    rosters,
    user_attendance,
    users,
    users_rosters,
);
