error_chain!{
    foreign_links {
        Config(::config::ConfigError);
        Handlebars(::handlebars::TemplateRenderError);
        Io(::std::io::Error);
        Json(::serde_json::Error);
        ParseBool(::std::str::ParseBoolError);
        ParseInt(::std::num::ParseIntError);
        Regex(::regex::Error);
        Reqwest(::reqwest::Error);
    }
}
