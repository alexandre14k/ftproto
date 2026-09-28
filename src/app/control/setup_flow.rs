// src/app/control/setup_flow.rs

use std::thread;

use crate::app::*;

pub enum SetupOutcome {
    Apply(FtpConfig),
    Cancel,
    Pending,
}

pub fn control_begin_setup() {
    if model_setup_active() {
        return;
    }
    let cfg = model_config_snapshot();
    model_set_setup(SetupFlow {
        stage: SetupStage::Port,
        listen_ip: cfg.listen_ip.clone(),
        listen_port: cfg.listen_port,
        files_root: cfg.files_root.clone(),
    });
    view_setup_intro();
    view_setup_prompt_port(cfg.listen_port);
}

pub fn control_feed_setup(line: &str) {
    let input = line.trim().to_string();
    let outcome =
        model_update_setup(|f| setup_advance(f, &input));
    match outcome {
        Some(SetupOutcome::Apply(cfg)) => {
            let _ = model_take_setup();
            thread::spawn(move || control_apply_setup(cfg));
        }
        Some(SetupOutcome::Cancel) => {
            let _ = model_take_setup();
            view_setup_cancelled();
        }
        _ => {}
    }
}