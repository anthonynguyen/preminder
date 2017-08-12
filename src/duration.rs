use chrono::Duration;

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
