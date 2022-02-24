use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameter {
    pub container_id: String,
    pub root_path: String,
    pub args: Vec<String>,
    pub bundle: String,
    pub pid_file: String,
}