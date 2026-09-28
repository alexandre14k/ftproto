// src/app/model/state.rs

use std::fs;
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Mutex, OnceLock};

use super::config::FtpConfig;
use super::runtime::ServerRuntime;
use super::setup::SetupFlow;

pub struct AppModel {
    pub config: FtpConfig,
    pub running: bool,
    pub events: Vec<String>,
    pub next_conn_id: u32,
    pub event_tx: Sender<String>,
    pub event_rx: Mutex<Receiver<String>>,
    pub runtime: Option<ServerRuntime>,
    pub setup: Option<SetupFlow>,
}

static APP_MODEL: OnceLock<Mutex<AppModel>> = OnceLock::new();

pub fn model_lock() -> &'static Mutex<AppModel> {
    APP_MODEL.get_or_init(|| {
        let (tx, rx) = mpsc::channel();
        Mutex::new(AppModel {
            config: FtpConfig::ftp_default_config(),
            running: false,
            events: Vec::new(),
            next_conn_id: 1,
            event_tx: tx,
            event_rx: Mutex::new(rx),
            runtime: None,
            setup: None,
        })
    })
}

pub fn model_config_snapshot() -> FtpConfig {
    let m = model_lock().lock().unwrap();
    FtpConfig {
        listen_ip: m.config.listen_ip.clone(),
        listen_port: m.config.listen_port,
        files_root: m.config.files_root.clone(),
    }
}

pub fn model_set_config(cfg: FtpConfig) {
    model_lock().lock().unwrap().config = cfg;
}

pub fn model_running() -> bool {
    model_lock().lock().unwrap().running
}

pub fn model_set_running(v: bool) {
    model_lock().lock().unwrap().running = v;
}

pub fn model_next_conn_id() -> u32 {
    let mut m = model_lock().lock().unwrap();
    let id = m.next_conn_id;
    m.next_conn_id += 1;
    id
}

pub fn model_prepare_files_root(
    root: &str,
) -> Result<(), String> {
    fs::create_dir_all(root).map_err(|e| e.to_string())
}