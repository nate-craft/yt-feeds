use chrono::{DateTime, Days, Local};

pub fn time_since_formatted(date: DateTime<Local>) -> String {
    // With yt-dlp --flat-playlist days are estimated and are often rounded down.
    // This makes them slightly more accurate
    let duration = Local::now().signed_duration_since(date.checked_sub_days(Days::new(1)).unwrap());

    if duration.num_days() >= 365 {
        let years = duration.num_days() / 365;
        if years > 1 {
            format!("{} Years Ago", years)
        } else {
            "1 Year Ago".to_string()
        }
    } else if duration.num_days() >= 28 {
        let months = duration.num_days() / 28;
        if months > 1 {
            format!("{} Months Ago", months)
        } else {
            "1 Month Ago".to_string()
        }
    } else if duration.num_days() >= 7 {
        let weeks = duration.num_days() / 7;
        if weeks > 1 {
            format!("{} Weeks Ago", weeks)
        } else {
            "1 Week Ago".to_string()
        }
    } else if duration.num_days() > 0 {
        if duration.num_days() > 1 {
            format!("{} Days Ago", duration.num_days())
        } else {
            "1 Day Ago".to_string()
        }
    } else {
        "Today".to_string()
    }
}
