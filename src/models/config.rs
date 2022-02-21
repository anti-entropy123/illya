use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct OCIRoot {
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OCIConfig {
    root: OCIRoot,
}