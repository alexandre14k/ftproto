// src/app/control/stop.rs

use std::sync::atomic::Ordering;
use std::thread;

use crate::app::*;

pub fn control_stop_server_async() {
    thread::spawn(control_stop_server_sync);
}

pub fn control_stop_server_sync() {
    if !model_running() {
        model_send_event("stop ignored: server not running");
        return;
    }
    let rt = match model_take_runtime() {
        Some(rt) => rt,
        None => {
            model_send_event(
                "stop ignored: server still starting",
            );
            return;
        }
    };
    model_set_running(false);
    let closed = server_stop_runtime(rt);
    model_send_event(&format!(
        "server stopped, {} connection(s) closed",
        closed
    ));
}

pub fn control_halt_runtime() {
    let rt = match model_take_runtime() {
        Some(rt) => rt,
        None => {
            model_set_running(false);
            return;
        }
    };
    model_set_running(false);
    let closed = server_stop_runtime(rt);
    model_send_event(&format!(
        "setup: server stopped, {} connection(s) closed",
        closed
    ));
}

pub fn server_stop_runtime(rt: ServerRuntime) -> usize {
    rt.stop.store(true, Ordering::Relaxed);
    let _ = rt.accept_thread.join();
    let handles: Vec<_> =
        rt.registry.lock().unwrap().drain(..).collect();
    let closed = handles.len();
    for h in handles {
        let _ = h.join();
    }
    closed
}

pub fn control_shutdown() {
    if model_running() {
        if let Some(rt) = model_take_runtime() {
            model_set_running(false);
            server_stop_runtime(rt);
        }
    }
    println!("bye");
}