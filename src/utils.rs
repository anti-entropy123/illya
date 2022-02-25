use log::info;

pub fn abs_path(path: &str) -> Result<String, &str> {
    match path.chars().nth(0) {
        // start with '/', it is abs path;
        Some(c) if c == std::path::MAIN_SEPARATOR => Ok(String::from(path)),
        // start without '/'
        Some(_) => {
            let mut cwd = std::env::var("PWD").expect("can't get PWD");
            cwd.push(std::path::MAIN_SEPARATOR);
            Ok(cwd + path)
        }
        // not have any char, error!
        _ => Err("path must not be empty"),
    }
}

pub fn last_must_separator(mut path: String) -> String {
    if path.is_empty() {
        return format!("{}", std::path::MAIN_SEPARATOR);
    }
    if path.as_str().chars().nth(path.len() - 1).unwrap() != std::path::MAIN_SEPARATOR {
        path.push(std::path::MAIN_SEPARATOR);
    }
    path
}

pub fn is_directory(path: &String) -> bool {
    std::path::Path::new(path).is_dir()
}

pub fn is_exist(path: &String) -> bool {
    std::path::Path::new(path).exists()
}

pub fn dir_item_names(dir: &String) -> Result<Vec<String>, ()> {
    let dir = match std::fs::read_dir(&dir) {
        Err(_) => return Err(()),
        Ok(v) => v,
    };
    let items: Vec<_> = dir
        .map(|x| String::from(x.unwrap().file_name().to_str().unwrap()))
        .collect();
    Ok(items)
}

pub fn display_cwd_items() {
    let cwd = std::env::current_dir().unwrap();
    let items = dir_item_names(&cwd.to_str().unwrap().to_string()).expect("can't access cwd");
    info!(
        "current is {}, has items: {}",
        cwd.to_str().unwrap(),
        items.join(" ")
    );
}

pub fn now_utc() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
