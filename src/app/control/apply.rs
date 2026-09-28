// src/app/control/apply.rs

use crate::app::*;

pub fn control_apply_setup(cfg: FtpConfig) {
    let was_running = model_running();
    if was_running {
        control_halt_runtime();
    }
    if let Err(e) = model_prepare_files_root(&cfg.files_root) {
        model_send_event(&format!(
            "setup failed: folder {}: {}",
            cfg.files_root, e
        ));
        return;
    }
    model_send_event(&format!(
        "setup: settings {}:{} root {}",
        cfg.listen_ip,
        cfg.listen_port,
        cfg.files_root,
    ));
    model_set_config(cfg);
    if was_running {
        control_start_server();
        model_send_event("setup: server restarted");
    }
}