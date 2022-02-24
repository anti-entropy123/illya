use {
    crate::{
        commands::Context,
        models::{oci, state},
    },
    log::warn,
    serde_json as json,
    std::{fs, io::prelude::*, path},
};

pub enum Status {
    Loaded,
    Created,
    Running,
    // Paused,
    // Restored,
    Stopped,
}

impl Status {
    pub fn to_string(&self) -> String {
        let s = match self {
            Status::Loaded => "loaded",
            Status::Created => "created",
            Status::Running => "running",
            Status::Stopped => "stopped",
        };
        s.to_string()
    }
}

pub struct Container {
    pub id: String,
    ctx: Box<Context>,
}

impl Container {
    pub fn new(container_id: &String, ctx: Box<Context>) -> Self {
        Container {
            id: container_id.clone(),
            ctx: ctx,
        }
    }

    pub fn update_state_file(&self, state: &state::State) {
        let crt_dir = &self.crt_dir();
        let crt_dir = path::Path::new(crt_dir);
        if !crt_dir.is_dir() {
            fs::create_dir_all(crt_dir).expect("failed to create crt dir");
        }
        let mut state_file = fs::File::options()
            .write(true)
            .create(true)
            .open(self.state_path())
            .expect("failed to open state.json");

        let val = json::ser::to_string(state).expect("failed to serialize state");
        write!(state_file, "{}", val).expect("failed write to state.json");
    }

    pub fn load_state_file(&self) -> Result<state::State, String> {
        let state_path = &self.state_path();
        let state_path = path::Path::new(state_path);
        if !state_path.is_file() {
            return Err("container is not running".to_string());
        }
        let mut state_file = fs::File::open(state_path).unwrap();
        let mut val = String::new();
        if let Err(e) = state_file.read_to_string(&mut val) {
            return Err(format!("failed to read state.json: {}", e));
        }
        let state: state::State = match json::from_str(&val) {
            Ok(obj) => obj,
            Err(e) => return Err(format!("failed to deserialize state.json: {}", e)),
        };
        return Ok(state);
    }

    pub fn crt_dir(&self) -> String {
        let rt_dir = path::PathBuf::from(&self.ctx.runtime_dir);
        let container_rt_dir = rt_dir.join("containers").join(&self.id);
        // "/run/user/1000/illya/containers/<container-id>/"
        String::from(container_rt_dir.to_str().unwrap())
    }

    pub fn fifo_path(&self) -> String {
        path::Path::new(&self.crt_dir())
            .join("exec.fifo")
            .to_str()
            .expect("failed to get exec.fifo path")
            .to_string()
    }

    pub fn state_path(&self) -> String {
        path::Path::new(&self.crt_dir())
            .join("state.json")
            .to_str()
            .expect("failed to get state.json path")
            .to_string()
    }

    pub fn status(&self) -> Status {
        if path::Path::new(&self.fifo_path()).is_file() {
            return Status::Created;
        }
        if path::Path::new(&self.state_path()).is_file() {
            return Status::Running;
        }
        return Status::Stopped;
    }

    pub fn clear(&self) {
        let crt_dir = self.crt_dir();
        if let Err(e) = fs::remove_dir_all(path::Path::new(&crt_dir)) {
            warn!("failed to clear container runtime directory: {}", e);
        }
    }
}

pub fn load_oci_config(bundle: &String) -> json::Result<oci::Config> {
    let config_path = String::from(bundle) + "config.json";
    let mut config_file = fs::File::open(config_path)
        .expect(format!("can't open config.json in {}", bundle).as_str());
    let mut config: String = String::new();
    config_file
        .read_to_string(&mut config)
        .expect("read config.json fail");
    let config: oci::Config = json::from_str(&config)?;
    Ok(config)
}
