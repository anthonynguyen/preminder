error_chain!{
    foreign_links {
        Config(::config::ConfigError);
        Io(::std::io::Error);
        ParseInt(::std::num::ParseIntError);
        Reqwest(::reqwest::Error);
    }
}
