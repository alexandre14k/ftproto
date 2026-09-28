// src/app/control/accept.rs

use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

use crate::app::*;

pub fn accept_loop(
    listener: TcpListener,
    stop: StopFlag,
    registry: ConnRegistry,
    root: PathBuf,
) {
    while !stop.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, _)) => {
                accept_spawn_session(
                    stream,
                    &stop,
                    &registry,
                    &root,
                );
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(50));
            }
        }
        registry
            .lock()
            .unwrap()
            .retain(|h| !h.is_finished());
    }
}

fn accept_spawn_session(
    stream: TcpStream,
    stop: &StopFlag,
    registry: &ConnRegistry,
    root: &PathBuf,
) {
    let peer = stream
        .peer_addr()
        .map(|a| a.to_string())
        .unwrap_or_else(|_| String::from("unknown"));
    let id = model_next_conn_id();
    model_send_event(&format!(
        "conn #{} opened from {}",
        id, peer
    ));
    let sess_stop = stop.clone();
    let sess_root = root.clone();
    let handle = thread::spawn(move || {
        control_session_run(id, stream, sess_root, sess_stop)
    });
    registry.lock().unwrap().push(handle);
}