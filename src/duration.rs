use chrono::{DateTime, Duration, TimeZone};

use errors::*;

pub fn parse(st: &str) -> Result<Duration> {
    let mut st = st.to_owned();
    let last_char = st.pop().ok_or("Empty period!")?;
    let num = st.parse::<u64>()?;

    match last_char {
        's' => Ok(Duration::seconds(num as i64)),
        'm' => Ok(Duration::minutes(num as i64)),
        'h' => Ok(Duration::hours(num as i64)),
        'd' => Ok(Duration::days(num as i64)),
        'w' => Ok(Duration::weeks(num as i64)),
        _ => Err(Error::from("Invalid type for period"))
    }
}

fn approx(coarse: &Duration, exact: &Duration) -> String {
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

pub fn nice(dura: Duration) -> String {
    let periods: Vec<(i64, &str, fn(i64) -> Duration)> = vec![
        (dura.num_weeks(), "week", Duration::weeks),
        (dura.num_days(), "day", Duration::days),
        (dura.num_hours(), "hour", Duration::hours),
        (dura.num_minutes(), "minute", Duration::minutes)
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

pub fn relative<T: TimeZone>(before: DateTime<T>, after: DateTime<T>) -> String {
    let diff = after.signed_duration_since::<T>(before);
    format!("{} ago", nice(diff))
}
