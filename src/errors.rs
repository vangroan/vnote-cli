error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    foreign_links {
        Fmt(::std::fmt::Error);
        Io(::std::io::Error);
        Regex(::regex::Error);
        Yaml(::serde_yaml::Error);
    }
}
