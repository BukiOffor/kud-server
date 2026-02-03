use super::*;
use crate::dto::events::{CheckIntoEventRequest, CreateEventRequest, UpdateEventRequest};
use crate::models::{events::Event, user_attendance::UserAttendance};
use chrono::{Duration, Local};

pub async fn create_event(
    pool: Arc<Pool>,
    user_id: Uuid,
    payload: CreateEventRequest,
) -> Result<Event, ModuleError> {
    let mut conn = pool.get().await?;

    let event = Event {
        id: Uuid::now_v7(),
        title: payload.title,
        description: payload.description,
        date: payload.date,
        time: payload.time,
        location: payload.location,
        created_by: user_id,
        attendance_type: payload.attendance_type,
        grace_period_in_minutes: payload.grace_period_in_minutes,
        created_at: Local::now().naive_local(),
        updated_at: Local::now().naive_local(),
    };

    diesel::insert_into(schema::events::table)
        .values(&event)
        .execute(&mut conn)
        .await?;

    Ok(event)
}

pub async fn update_event(
    pool: Arc<Pool>,
    payload: UpdateEventRequest,
) -> Result<Event, ModuleError> {
    let mut conn = pool.get().await?;

    let event = diesel::update(schema::events::table.find(payload.event_id))
        .set((
            payload.title.as_ref().map(|t| schema::events::title.eq(t)),
            payload
                .description
                .as_ref()
                .map(|d| schema::events::description.eq(d)),
            payload.date.as_ref().map(|d| schema::events::date.eq(d)),
            payload.time.as_ref().map(|t| schema::events::time.eq(t)),
            payload
                .location
                .as_ref()
                .map(|l| schema::events::location.eq(l)),
            payload
                .attendance_type
                .as_ref()
                .map(|a| schema::events::attendance_type.eq(a)),
            payload
                .grace_period_in_minutes
                .as_ref()
                .map(|g| schema::events::grace_period_in_minutes.eq(g)),
            schema::events::updated_at.eq(Local::now().naive_local()),
        ))
        .get_result::<Event>(&mut conn)
        .await?;

    Ok(event)
}

pub async fn delete_event(pool: Arc<Pool>, event_id: Uuid) -> Result<Message, ModuleError> {
    let mut conn = pool.get().await?;

    let count = diesel::delete(schema::events::table)
        .filter(schema::events::id.eq(event_id))
        .execute(&mut conn)
        .await?;

    if count == 0 {
        return Err(ModuleError::Error("Event not found".into()));
    }

    Ok(Message::new("Event deleted successfully"))
}

pub async fn check_into_event(
    pool: Arc<Pool>,
    payload: CheckIntoEventRequest,
) -> Result<Message, ModuleError> {
    let mut conn = pool.get().await?;

    let event: Event = schema::events::table
        .find(payload.event_id)
        .first(&mut conn)
        .await
        .map_err(|_| ModuleError::Error("Event not found".into()))?;

    // Check check-in logic
    let now = Local::now().naive_local();

    let start_time = event.date.and_time(event.time);
    let end_time = start_time + Duration::minutes(event.grace_period_in_minutes as i64);

    // Simple window check
    if now < start_time {
        // Optionally allow checking in slightly early? User didn't specify. Strict for now.
        return Err(ModuleError::Error("Event has not started yet".into()));
    }

    if now > end_time {
        return Err(ModuleError::Error(
            "Event check-in window has closed".into(),
        ));
    }

    let today = now.date();
    let mut attendance = UserAttendance::new(payload.user_id, today);
    attendance.set_event_id(event.id);
    attendance.set_attendance_type(payload.attendance_type);

    diesel::insert_into(schema::user_attendance::table)
        .values(&attendance)
        .execute(&mut conn)
        .await?;

    Ok(Message::new("Checked in successfully"))
}

pub async fn get_event(pool: Arc<Pool>, event_id: Uuid) -> Result<Event, ModuleError> {
    let mut conn = pool.get().await?;
    let event = schema::events::table
        .find(event_id)
        .first::<Event>(&mut conn)
        .await
        .map_err(|_| ModuleError::Error("Event not found".into()))?;
    Ok(event)
}

pub async fn get_events(pool: Arc<Pool>) -> Result<Vec<Event>, ModuleError> {
    let mut conn = pool.get().await?;
    let events = schema::events::table.load::<Event>(&mut conn).await?;
    Ok(events)
}

pub async fn get_events_by_user(pool: Arc<Pool>, user_id: Uuid) -> Result<Vec<Event>, ModuleError> {
    let mut conn = pool.get().await?;
    // Assuming "events by user" means events created by the user, or events the user attended?
    // Given the context of "Created By" in schema, implies admin created events.
    // If user_id is the creator:
    let events = schema::events::table
        .filter(schema::events::created_by.eq(user_id))
        .load::<Event>(&mut conn)
        .await?;
    Ok(events)
}

pub async fn get_upcoming_events(pool: Arc<Pool>) -> Result<Vec<Event>, ModuleError> {
    let mut conn = pool.get().await?;
    let now = Local::now().naive_local();
    let events = schema::events::table
        .filter(
            schema::events::date.gt(now.date()).or(schema::events::date
                .eq(now.date())
                .and(schema::events::time.gt(now.time()))),
        )
        .load::<Event>(&mut conn)
        .await?;
    Ok(events)
}

pub async fn get_past_events(pool: Arc<Pool>) -> Result<Vec<Event>, ModuleError> {
    let mut conn = pool.get().await?;
    let now = Local::now().naive_local();
    let events = schema::events::table
        .filter(
            schema::events::date.lt(now.date()).or(schema::events::date
                .eq(now.date())
                .and(schema::events::time.lt(now.time()))),
        )
        .load::<Event>(&mut conn)
        .await?;
    Ok(events)
}
