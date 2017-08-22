error_chain!{
    links {
        Tera(::tera::Error, ::tera::ErrorKind);
    }

    foreign_links {
        ChronoParser(::chrono::ParseError);
        Config(::config::ConfigError);
        Io(::std::io::Error);
        Json(::serde_json::Error);
        Lettre(::lettre::email::error::Error);
        LettreSmtp(::lettre::transport::smtp::error::Error);
        ParseBool(::std::str::ParseBoolError);
        ParseInt(::std::num::ParseIntError);
        Regex(::regex::Error);
        Reqwest(::reqwest::Error);
        StringFromUtf8(::std::string::FromUtf8Error);
    }
}
