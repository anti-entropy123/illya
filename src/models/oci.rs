use {
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Process {
    pub args: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Root {
    pub path: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub root: Root,
    pub process: Process,
    pub annotations: HashMap<String, String>,
}
