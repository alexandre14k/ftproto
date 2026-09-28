// src/app/view/status.rs

use crate::app::*;

pub fn view_show_status(
    running: bool,
    cfg: &FtpConfig,
    conns: usize,
    events: &[String],
) {
    if running {
        println!(
            "server     : running on {}:{}",
            cfg.listen_ip, cfg.listen_port
        );
        println!("connections: {}", conns);
    } else {
        println!("server     : stopped");
        println!(
            "configured : {}:{}",
            cfg.listen_ip, cfg.listen_port
        );
    }
    println!("files root : {}", cfg.files_root);
    println!("user       : ftproto");
    println!("--- events ---");
    if events.is_empty() {
        println!("(none)");
        return;
    }
    for e in events {
        println!("{}", e);
    }
}