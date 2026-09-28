// src/app/model/runtime.rs

use std::thread::JoinHandle;

use super::state::model_lock;
use super::{ConnRegistry, StopFlag};

pub struct ServerRuntime {
    pub stop: StopFlag,
    pub registry: ConnRegistry,
    pub accept_thread: JoinHandle<()>,
}

pub fn model_set_runtime(rt: ServerRuntime) {
    model_lock().lock().unwrap().runtime = Some(rt);
}

pub fn model_take_runtime() -> Option<ServerRuntime> {
    model_lock().lock().unwrap().runtime.take()
}

pub fn model_conn_count() -> usize {
    let registry = {
        let m = model_lock().lock().unwrap();
        m.runtime.as_ref().map(|r| r.registry.clone())
    };
    match registry {
        Some(r) => {
            let mut g = r.lock().unwrap();
            g.retain(|h| !h.is_finished());
            g.len()
        }
        None => 0,
    }
}