use {
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
};

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

pub fn new_labels(annos: &HashMap<String, String>, bundle: &String) -> Vec<String> {
    let mut labels = vec![];
    for (k, v) in annos {
        labels.push(k.clone() + "=" + v.as_str());
    }
    labels.push(format!("bundle={}", bundle));
    labels
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
