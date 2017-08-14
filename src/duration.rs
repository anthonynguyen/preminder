use chrono;

use errors::*;

pub fn parse(st: &str) -> Result<chrono::Duration> {
    let mut st = st.to_owned();
    let last_char = st.pop().ok_or("Empty duration!")?;
    let num = st.parse::<u64>()?;

    match last_char {
        's' => Ok(chrono::Duration::seconds(num as i64)),
        'm' => Ok(chrono::Duration::minutes(num as i64)),
        'h' => Ok(chrono::Duration::hours(num as i64)),
        'd' => Ok(chrono::Duration::days(num as i64)),
        'w' => Ok(chrono::Duration::weeks(num as i64)),
        _ => Err(Error::from("Invalid type for duration"))
    }
}

fn approx(coarse: &chrono::Duration, exact: &chrono::Duration) -> String {
    if coarse.num_seconds() != exact.num_seconds() {
        "~".to_owned()
    } else {
        "".to_owned()
    }
}

fn plural(num: i64) -> String {
    if num != 1 {
        "s".to_owned()
    } else {
        "".to_owned()
    }
}

pub fn nice(dura: chrono::Duration) -> String {
    type DurationConstructor = fn(i64) -> chrono::Duration;
    let periods: Vec<(i64, &str, DurationConstructor)> = vec![
        (dura.num_weeks(), "week", chrono::Duration::weeks),
        (dura.num_days(), "day", chrono::Duration::days),
        (dura.num_hours(), "hour", chrono::Duration::hours),
        (dura.num_minutes(), "minute", chrono::Duration::minutes)
    ];

    let mut skip = true;

    for p in &periods {
        if p.0 > 0 {
            if skip {
                skip = false;
                continue;
            }
            let coarse = p.2(p.0);
            return format!("{}{} {}{}", approx(&coarse, &dura), p.0, p.1, plural(p.0));
        }
    }

    let seconds = dura.num_seconds();
    format!("{} second{}", seconds, plural(seconds))
}

pub fn relative<T: chrono::TimeZone>(before: chrono::DateTime<T>, after: chrono::DateTime<T>) -> String {
    let diff = after.signed_duration_since::<T>(before);
    format!("{} ago", nice(diff))
}

pub fn relative_helper(timestamp: &str) -> Result<String> {
    let timestamp = timestamp
        .parse::<chrono::DateTime<chrono::Utc>>()?
        .with_timezone::<chrono::offset::Local>(&chrono::offset::Local);

    Ok(relative::<chrono::offset::Local>(timestamp, chrono::Local::now()))
}
