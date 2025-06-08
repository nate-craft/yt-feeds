use chrono::{DateTime, Days, Local};
use crossterm::style::{Attribute, Color, StyledContent, Stylize};

fn apply_style(
    text: &str,
    color: Option<Color>,
    underline: bool,
    bold: bool,
) -> StyledContent<String> {
    let mut styled_text = text.to_string().with(Color::Reset);

    if let Some(c) = color {
        styled_text = styled_text.with(c);
    }

    if underline {
        styled_text = styled_text.attribute(Attribute::Underlined);
    }

    if bold {
        styled_text = styled_text.attribute(Attribute::Bold);
    }

    styled_text
}

pub fn highlight_query(str: &str, query: Option<&str>, color: Option<Color>) -> String {
    if let Some(query) = query {
        if let Some(pos) = str.to_lowercase().find(&query.to_lowercase()) {
            let before = apply_style(&str[..pos], color, false, false);
            let matched = apply_style(&str[pos..pos + query.len()], Some(Color::Red), true, true);
            let after = apply_style(&str[pos + query.len()..], color, false, false);

            return format!("{}{}{}", before, matched, after);
        }
    }
    return apply_style(str, color, false, false).to_string();
}

pub fn time_formatted_short(time_second: Option<i32>) -> String {
    if let Some(time_second) = time_second {
        let mut formatted = String::with_capacity(14);

        let hours = time_second / 3600;
        let minutes = (time_second % 3600) / 60;
        let seconds = time_second % 60;

        if hours > 0 {
            formatted.push_str(&hours.to_string());
            formatted.push('h');
        }
        if minutes > 0 {
            formatted.push_str(&minutes.to_string());
            formatted.push('m');
        }
        if time_second > 0 || formatted.is_empty() {
            formatted.push_str(&seconds.to_string());
            formatted.push('s');
        }
        formatted.push_str(" Watched");
        formatted
    } else {
        "Not Watched".to_string()
    }
}

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
