error_chain!{
    foreign_links {
        Config(::config::ConfigError);
        Io(::std::io::Error);
        Reqwest(::reqwest::Error);
    }
}
