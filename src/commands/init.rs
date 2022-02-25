use {
    crate::{
        commands::{Context, Executable},
        container,
        models::{init, state},
        utils,
    },
    clap::App,
    log::{error, warn},
    nix::sys::socket,
    serde_json,
    std::{env, fs, io, io::prelude::*, os::unix::io::FromRawFd, path, process},
    unix_named_pipe, wasmtime, wasmtime_wasi,
    wasmtime_wasi::sync::WasiCtxBuilder,
};

pub fn subcommand<'a>() -> App<'a> {
    App::new("init").about("init container").version("0.1")
}

pub struct Command {
    param: init::Parameter,
    container: container::Container,
}

pub fn new(_sub_matchs: &clap::ArgMatches, ctx: Context) -> Box<dyn Executable> {
    let pipe_fd_var =
        env::var("_LIBCONTAINER_INITPIPE").expect("can't get env _LIBCONTAINER_INITPIPE");
    let pipe_fd = pipe_fd_var
        .parse::<i32>()
        .expect(&format!("wrong format, pipe_fd_var={}", pipe_fd_var));

    let paramater: init::Parameter =
        serde_json::from_str(&read_from_pipe_fd(pipe_fd)).expect("failed format param to struct");

    Box::from(Command {
        container: container::Container::new(&paramater.container_id, Box::new(ctx)),
        param: paramater,
    })
}

impl Command {
    fn create_oci_log(&self) -> io::Result<fs::File> {
        let pid_file = path::Path::new(&self.param.pid_file);
        let dir = pid_file.parent().unwrap();
        fs::File::create(dir.join("oci-log"))
    }

    fn update_pid_file(&self) -> io::Result<()> {
        let mut pid_file = fs::File::create(&self.param.pid_file).expect("open pid-file fail");
        pid_file.write_all(format!("{}", process::id()).as_bytes())
    }

    fn wait_start(&self) {
        if !utils::is_directory(&self.container.crt_dir()) {
            fs::create_dir_all(&self.container.crt_dir())
                .expect("failed to create container_rt_dir");
        }

        let exec_fifo = path::Path::new(&self.container.crt_dir()).join("exec.fifo");
        unix_named_pipe::create(&exec_fifo, Some(0o740)).expect("failed to create fifo");
        // info!("create fifo ready, start to read it");
        fs::File::open(&exec_fifo)
            .expect("failed to open fifo")
            .read(&mut [0; 0])
            .expect("failed to read from fifo");
        // info!("successed to read from fifo");
        if let Err(e) = fs::remove_file(&exec_fifo) {
            warn!("could not remove exec.fifo: {}", e)
        }
    }
}

fn load_wasm_module(
    entry_point: &path::Path,
) -> (wasmtime::Store<wasmtime_wasi::WasiCtx>, wasmtime::Func) {
    let engine = wasmtime::Engine::default();
    let module =
        wasmtime::Module::from_file(&engine, entry_point).expect("failed to create wasm module");
    let mut linker = wasmtime::Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |cx| cx).expect("failed to add wasi to linker");
    let wasi_ctx = WasiCtxBuilder::new().inherit_stdio().build();
    let mut store = wasmtime::Store::new(&engine, wasi_ctx);
    let instance = linker
        .instantiate(&mut store, &module)
        .expect("failed to create wasm instance");

    let func = instance
        .get_func(&mut store, "_start")
        .expect("`_start` was not an exported function");

    (store, func)
}

fn call_wasm_func(mut store: wasmtime::Store<wasmtime_wasi::WasiCtx>, func: wasmtime::Func) {
    let func = func.typed::<(), (), _>(&store).expect("wrong func type");
    func.call(&mut store, ()).expect("failed to call func");
    // println!("result: {:?}", result);
}

fn read_from_pipe_fd(pipe_fd: i32) -> String {
    let mut buffer = [0u8; 1024];
    let mut init_pipe = unsafe { fs::File::from_raw_fd(pipe_fd) };
    let mut raw_data: Vec<u8> = vec![];
    loop {
        match init_pipe.read(&mut buffer) {
            Ok(size) if size > 0 => {
                raw_data.extend(&buffer[0..size]);
                if size < buffer.len() {
                    break;
                }
            }
            Ok(_) => break,
            Err(e) => {
                error!("read init_pipe fail, {}", e);
                break;
            }
        }
    }
    String::from_utf8(raw_data).expect("can't from_utf8")
}

fn create_attach_sock() -> Result<socket::SockAddr, String> {
    let attach_sock = socket::socket(
        socket::AddressFamily::Unix,
        socket::SockType::Stream,
        socket::SockFlag::empty(),
        None,
    )
    .expect("create sock fail");

    let sockaddr = match nix::sys::socket::SockAddr::new_unix("attach") {
        Ok(addr) => addr,
        Err(e) => return Err(format!("can't create sock_addr: {}", e)),
    };
    if let Err(e) = socket::bind(attach_sock, &sockaddr) {
        return Err(format!("bind attach socket file fail: {}", e));
    };
    Ok(sockaddr)
}

impl Executable for Command {
    fn execute(&self) {
        let created_time = utils::now_utc();
        env::set_current_dir(&self.param.root_path).expect("failed to change cwd");
        // utils::display_cwd_items();
        // self.create_oci_log().expect("failed to create oci-log");
        self.update_pid_file().expect("failed to update pid file");
        let entry = path::Path::new(self.param.args.get(0).unwrap());
        let (s, f) = load_wasm_module(&entry);
        self.wait_start();

        let oci_config =
            container::load_oci_config(&self.param.bundle).expect("failed to load oci config");
        let labels = state::new_labels(&oci_config.annotations, &self.param.bundle);

        self.container.update_state_file(&state::State {
            id: self.container.id.clone(),
            init_process_pid: process::id(),
            created: created_time,
            config: state::Config {
                rootfs: self.param.root_path.to_string(),
                labels,
            },
        });
        call_wasm_func(s, f);
        self.container.clear();
    }
}
