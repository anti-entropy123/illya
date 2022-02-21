use clap::{App,};
use std::{
    env,
    process,
    os::unix::io::{FromRawFd},
    fs,
    fs::File,
    io::prelude::*
};
use log::{error, debug};
use super::{Executable, Context};

pub fn subcommand<'a>() -> App<'a> {
    App::new("init")
        .about("init container")
        .version("0.1")
}

#[derive(Debug)]
pub struct Command {
}

pub fn new (sub_matchs: &clap::ArgMatches) -> Box<dyn Executable> {
    Box::from(Command{})
}

impl Executable for Command {
    fn execute (&self,) {
        let mut buffer = [0u8; 1024];
        let pipe_fd_var = env::var("_LIBCONTAINER_INITPIPE").expect("can't get env _LIBCONTAINER_INITPIPE");
        let bundle = env::var("_CONTAINER_BUNDLE").expect("can't get env _CONTAINER_BUNDLE");
        env::set_current_dir(bundle).expect("fail to change cwd");
        debug!("current is {:?}", env::current_dir().unwrap());
        for item in fs::read_dir(env::current_dir().unwrap()).unwrap() {
            debug!("{:?}", item.unwrap());
        }
        let pipe_fd = pipe_fd_var.parse::<i32>().expect((format!("wrong format, pipe_fd_var={}", pipe_fd_var)).as_str());
        let mut init_pipe = unsafe { File::from_raw_fd(pipe_fd)};
        let mut raw_data: Vec<u8> = vec![];
        loop {
            match init_pipe.read(&mut buffer) {
                Ok(size) if size > 0 => {
                    raw_data.extend(&buffer[0..size]);
                    if size < buffer.len() {
                        break;
                    }
                },
                Ok(_) => {break},
                Err(e) => {
                    error!("read init_pipe fail, {}", e);
                    break;
                }
            }
        }
        let pid_file = String::from_utf8(raw_data).expect("can't from_utf8");
        let run_path = String::from(&pid_file[0..pid_file.len()-"pidfile".len()-1]);
        File::create((run_path + "oci-log").as_str()).expect("create oci-log fail");
        // prinln!("pid_file={}", pid_file);
        let mut pid_file = File::create(pid_file).expect("open pid-file fail");
        pid_file.write_all(format!("{}", process::id()).as_bytes()).expect("write pid-file fail");
        
        // let attach_sock = socket::socket(
        //                     socket::AddressFamily::Unix, 
        //                     socket::SockType::Stream, 
        //                     socket::SockFlag::empty(), 
        //                     None
        //                 ).expect("create sock fail");
        
        // utils::log(format!("current is {:?}", env::current_dir().expect("can't get cwd")));
        // for item in fs::read_dir(env::current_dir().unwrap()).unwrap() {
        //     utils::log(item.unwrap());
        // }

        // let sockaddr = match nix::sys::socket::SockAddr::new_unix("attach") {
        //     Ok(addr) => {addr},
        //     Err(e) => {
        //         utils::log(e);
        //         process::exit(1);
        //     },
        // };
        // match socket::bind(attach_sock, &sockaddr) {
        //     Ok(_) => {utils::log("success bind sockaddr")},
        //     Err(e) => {utils::log(format!("bind attach socket file fail: {}", e).as_str())}
        // };
        
    }

    
}