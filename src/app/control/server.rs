// src/app/control/server.rs

use std::net::{IpAddr, SocketAddr, TcpListener};
use std::path::PathBuf;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread;

use crate::app::*;

pub fn control_start_server() {
    if model_running() {
        model_send_event(
            "start ignored: server already running",
        );
        return;
    }
    model_set_running(true);
    thread::spawn(server_launch);
}

fn server_launch() {
    let cfg = model_config_snapshot();
    if let Err(e) = model_prepare_files_root(&cfg.files_root) {
        model_set_running(false);
        model_send_event(&format!(
            "start failed: folder {}: {}",
            cfg.files_root, e
        ));
        return;
    }
    let addr = server_bind_addr(&cfg);
    let listener = match TcpListener::bind(addr) {
        Ok(l) => l,
        Err(e) => {
            model_set_running(false);
            model_send_event(&format!(
                "start failed: bind {}: {}",
                addr, e
            ));
            return;
        }
    };
    let _ = listener.set_nonblocking(true);
    let stop: StopFlag = Arc::new(AtomicBool::new(false));
    let registry: ConnRegistry =
        Arc::new(Mutex::new(Vec::new()));
    let root = PathBuf::from(&cfg.files_root);
    let rt_stop = stop.clone();
    let rt_registry = registry.clone();
    let accept_thread =
        thread::spawn(move || {
            accept_loop(
                listener,
                stop,
                registry,
                root,
            )
        });
    model_set_runtime(ServerRuntime {
        stop: rt_stop,
        registry: rt_registry,
        accept_thread,
    });
    model_send_event(&format!(
        "server listening on {}",
        addr
    ));
}

fn server_bind_addr(cfg: &FtpConfig) -> SocketAddr {
    let ip: IpAddr = cfg
        .listen_ip
        .parse()
        .unwrap_or("0.0.0.0".parse().unwrap());
    SocketAddr::new(ip, cfg.listen_port)
}