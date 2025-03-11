use std::{
    fmt::Write,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use time::{
    OffsetDateTime, PrimitiveDateTime,
    format_description::well_known::{Rfc2822, Rfc3339},
};

pub struct TryFromSliceError(());
impl std::fmt::Debug for TryFromSliceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("TryFromSliceError").finish()
    }
}

pub fn s_to_primitive_dt(s: &str) -> sqlx::types::time::PrimitiveDateTime {
    match time::OffsetDateTime::parse(s, &Rfc3339) {
        Ok(parsed_date) => {
            sqlx::types::time::PrimitiveDateTime::new(parsed_date.date(), parsed_date.time())
        }
        Err(e) => {
            eprintln!("Error parsing date: {:?}", e);
            primitive_dt_null()
        }
    }
}

pub fn primitive_dt_null() -> sqlx::types::time::PrimitiveDateTime {
    sqlx::types::time::PrimitiveDateTime::new(
        sqlx::types::time::Date::from_ordinal_date(1970, 1).unwrap(),
        sqlx::types::time::Time::from_hms(0, 0, 0).unwrap(),
    )
}

pub fn primitive_to_iso8601_string(
    primitive_date_time: sqlx::types::time::PrimitiveDateTime,
) -> String {
    let time = primitive_date_time.time();

    let mut second = time.second();
    if time.millisecond() > 0 {
        second += 1;
    }

    let mut formatted_string = String::new();
    write!(
        formatted_string,
        "{:10}T{:02}:{:02}:{:02}Z",
        primitive_date_time.date(),
        time.hour(),
        time.minute(),
        second
    )
    .unwrap();

    formatted_string
}

pub fn current_iso8601_string() -> String {
    let now = time::OffsetDateTime::now_utc();

    let mut formatted_string = String::new();
    write!(
        formatted_string,
        "{:10}T{:02}:{:02}:{:02}Z",
        now.date(),
        now.time().hour(),
        now.time().minute(),
        now.time().second()
    )
    .unwrap();

    formatted_string
}

pub fn is_video(mime_type: &str) -> bool {
    mime_type.starts_with("video")
}

pub fn ext_from_mime_type(mime_type: &str) -> Option<&str> {
    match mime_type {
        "video/mp4" => Some("mp4"),
        "video/quicktime" => Some("mov"),
        "video/x-msvideo" => Some("avi"),
        "video/x-flv" => Some("flv"),
        "video/x-matroska" => Some("mkv"),
        "video/x-ms-wmv" => Some("wmv"),
        "video/x-ms-asf" => Some("asf"),
        "video/x-m4v" => Some("m4v"),
        "video/3gpp" => Some("3gp"),
        "video/3gpp2" => Some("3g2"),
        "video/webm" => Some("webm"),
        "video/ogg" => Some("ogv"),
        "video/avi" => Some("avi"),
        _ => None,
    }
}

pub fn time_ago(datetime: &PrimitiveDateTime) -> String {
    let system_time = datetime.assume_utc().into();
    let now = SystemTime::now();
    let duration = now.duration_since(system_time).unwrap_or(Duration::ZERO);

    let total_secs = duration.as_secs();

    if total_secs < 60 {
        return "just now".to_string(); // Less than a minute
    }
    let minutes = total_secs / 60;
    if minutes < 60 {
        return format!(
            "{} minute{} ago",
            minutes,
            if minutes > 1 { "s" } else { "" }
        );
    }
    let hours = minutes / 60;
    if hours < 24 {
        return format!("{}h ago", hours);
    }
    let days = hours / 24;
    if days < 30 {
        return format!("{}d ago", days);
    }
    let months = days / 30;
    if months < 12 {
        return format!("{}mo ago", months);
    }
    let years = months / 12;
    format!("{}y ago", years)
}

pub fn format_expiry(duration: Duration) -> String {
    let expiry_time = SystemTime::now() + duration;
    let datetime = OffsetDateTime::from_unix_timestamp(
        expiry_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64,
    )
    .expect("Invalid timestamp");
    datetime.format(&Rfc2822).expect("Failed to format date")
}
