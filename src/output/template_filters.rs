use std::collections::HashMap;

use chrono;
use serde_json::value::Value;
use tera;

use duration;

pub fn relative(value: Value, mut args: HashMap<String, Value>) -> tera::Result<Value> {
    let local = match args.remove("local") {
        Some(val) => try_get_value!("relative", "local", bool, val),
        None => false,
    };
    let datestr = try_get_value!("relative", "value", String, value);

    if local {
        Ok(
            duration::relative(
                datestr.parse::<chrono::DateTime<chrono::Local>>().map_err(
                    |err| {
                        tera::Error::from(format!("timestamp parse error: {}", err))
                    },
                )?,
                chrono::Local::now(),
            ).into(),
        )
    } else {
        Ok(
            duration::relative(
                datestr.parse::<chrono::DateTime<chrono::Utc>>().map_err(
                    |err| {
                        tera::Error::from(format!("timestamp parse error: {}", err))
                    },
                )?,
                chrono::Utc::now(),
            ).into(),
        )
    }
}
