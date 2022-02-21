

pub fn abs_path(path: &str) -> Result<String, &str> {
    match path.chars().nth(0)   {
        // start with '/', it is abs path;
        Some(c) if c == std::path::MAIN_SEPARATOR => {
            Ok(String::from(path))
        },
        // start without '/'
        Some(c) => {
            let mut cwd = std::env::var("PWD").expect("can't get PWD");
            cwd.push(std::path::MAIN_SEPARATOR);
            Ok(cwd + path)
        },
        // not have any char, error!
        _ => {Err("path must not be empty")}
    }
}

pub fn last_must_separator(mut path: String) -> String {
    if path.is_empty() {
        return format!("{}", std::path::MAIN_SEPARATOR);
    }
    if path.as_str().chars().nth(path.len()-1).unwrap() != std::path::MAIN_SEPARATOR {
        path.push(std::path::MAIN_SEPARATOR); 
    }
    path
}