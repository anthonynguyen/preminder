error_chain!{
    foreign_links {
        Config(::config::ConfigError);
        Io(::std::io::Error);
        Json(::serde_json::Error);
        ParseBool(::std::str::ParseBoolError);
        ParseInt(::std::num::ParseIntError);
        Reqwest(::reqwest::Error);
    }
}
