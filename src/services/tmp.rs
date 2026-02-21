fn is_within_attendance_window(now: chrono::DateTime<chrono_tz::Tz>) -> bool {
    let weekday = now.weekday();
    let hour = now.hour();
    let minute = now.minute();

    match weekday {
        chrono::Weekday::Sun => {
            // Sunday: anytime
            true
        }
        chrono::Weekday::Tue => {
            // Wednesday: 16:30 → 18:00
            let minutes_since_midnight = hour * 60 + minute;
            let start = 18 * 60 + 30; // 6:30 PM
            let end = 24 * 60; // 8:00 PM
            let is_day = minutes_since_midnight >= start && minutes_since_midnight <= end;

            is_day
        }
        chrono::Weekday::Wed => {
            // Wednesday: 16:30 → 18:00
            let minutes_since_midnight = hour * 60 + minute;
            let start = 16 * 60 + 30; // 4:30 PM
            let end = 18 * 60; // 6:00 PM
            minutes_since_midnight >= start && minutes_since_midnight <= end
        }
        _ => false, // All other days
    }
}