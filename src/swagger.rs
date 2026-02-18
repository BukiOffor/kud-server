use crate::dto;
use crate::handlers;
use crate::models;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::auth::auth,
        handlers::users::register_user,
        handlers::users::get_user,
        handlers::users::get_all_users,
        handlers::users::update_user,
        handlers::users::deactivate_user,
        handlers::users::activate_user,
        handlers::users::update_user_role,
        handlers::users::change_password,
        handlers::users::reset_user_device_id,
        handlers::users::admin_update_user,
        handlers::user_attendance::sign_attendance,
        handlers::user_attendance::admin_sign_attendance,
        handlers::user_attendance::get_attendance_on_day,
        handlers::user_attendance::revoke_attendance,
        handlers::events::create_event,
        handlers::events::update_event,
        handlers::events::delete_event,
        handlers::events::get_event,
        handlers::events::get_events,
        handlers::events::get_events_by_user,
        handlers::events::get_upcoming_events,
        handlers::events::get_past_events,
        handlers::events::check_into_event,
        handlers::events::check_into_event_with_identifier,
        handlers::analytics::get_total_users,
        handlers::analytics::get_users_present_on_day,
        handlers::analytics::get_upcoming_birthdays,
        handlers::analytics::get_attendance_rates,
        handlers::analytics::get_user_attendance,
        handlers::analytics::get_event_stats_report,
        handlers::roster::create_roster,
        handlers::roster::get_roster,
        handlers::roster::update_roster,
        handlers::roster::get_all_rosters,
        handlers::roster::activate_roster,
        handlers::roster::delete_roster,
        handlers::roster::view_roster_assignments,
        handlers::roster::update_user_hall,
        handlers::logs::get_logs,
        handlers::logs::get_user_activity,
    ),
    components(
        schemas(
            crate::auth::dto::LoginPayload,
            dto::user::UserDto,
            dto::user::NewUser,
            dto::user::UserFilter,
            dto::user::UpdateUserRequest,
            dto::user::UpdateUserRoleRequest,
            dto::user::AdminUpdateUserRequest,
            dto::user::ChangePasswordRequest,
            models::users::Role,
            dto::attendance::UserAttendanceDto,
            dto::attendance::AttendanceWithUser,
            dto::attendance::GeoPoint,
            dto::attendance::SignAttendanceRequest,
            dto::attendance::AdminSignAttendanceRequest,
            models::user_attendance::AttendanceType,
            dto::events::CreateEventRequest,
            dto::events::UpdateEventRequest,
            dto::events::CheckIntoEventRequest,
            dto::events::CheckInWithIdentifierRequest,
            models::events::Event,
            models::events::Location,
            dto::roster::NewRoster,
            dto::roster::RosterDto,
            dto::roster::UpdateRosterRequest,
            dto::roster::RosterAssignmentDto,
            models::roster::Roster,
            models::roster::Hall,
            dto::analytics::UserPresentStats,
            dto::analytics::AttendanceStats,
            dto::analytics::AttendanceSummary,
            dto::analytics::UserAttendanceHistory,
            dto::analytics::EventAttendee,
            dto::analytics::EventStatsReport,
            dto::pagination::Pagination,
            dto::pagination::Metadata,
            models::activity_logs::ActivityLog,
            models::activity_logs::ActivityLogResponse,
            models::activity_logs::ActivityType,
            dto::MessageEmpty,
            dto::MessageString,
            dto::MessageAttendanceVec,
            dto::MessageUserDtoVec,
            dto::MessageUserPresentStats,
            dto::MessageAttendanceStats,
            dto::MessageUserAttendanceHistory,
            dto::MessageEventStatsReport,
            dto::MessageRosterDto,
            dto::MessageRosterAssignmentDtoVec,
            dto::roster::UpdateUserHallRequest,
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints"),
        (name = "users", description = "User management endpoints"),
        (name = "attendance", description = "Attendance tracking endpoints"),
        (name = "events", description = "Event management endpoints"),
        (name = "analytics", description = "Analytics and reporting endpoints"),
        (name = "roster", description = "Roster management endpoints"),
        (name = "logs", description = "Activity logging endpoints"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

// impl utoipa::Modify for SecurityAddon {
//     fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
//         let components = openapi.components.as_mut().unwrap();
//         components.add_security_scheme(
//             "jwt",
//             utoipa::openapi::security::SecurityScheme::Http(
//                 utoipa::openapi::security::HttpBuilder::new()
//                     .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
//                     .bearer_format("JWT")
//                     .build(),
//             ),
//         );
//     }
// }

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "access_token",
                utoipa::openapi::security::SecurityScheme::ApiKey(
                    utoipa::openapi::security::ApiKey::Cookie(
                        utoipa::openapi::security::ApiKeyValue::new("access_token"),
                    ),
                ),
            )
        }
    }
}

pub fn swagger_routes() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api/v1/openapi.json", ApiDoc::openapi())
}
