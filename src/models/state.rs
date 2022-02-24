use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub rootfs: String,
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct State {
    pub id: String,
    pub init_process_pid: u32,
    pub created: String,
    pub config: Config,
}

impl State {
    pub fn annotations(&self) -> (Vec<String>, String) {
        let mut annos: Vec<String> = vec![];
        let mut bundle: String = String::new();
        for label in &self.config.labels {
            match label.split_once("=") {
                Some((s1, s2)) if s1 == "bundle" => {
                    bundle = s2.to_string();
                }
                _ => {
                    annos.push(label.clone());
                }
            }
        }
        (annos, bundle)
    }
}
