// src/app/control/dispatch.rs

use std::io::{self, BufRead, Write};
use std::thread;

use crate::app::*;

pub fn control_main() -> Result<(), String> {
    view_show_menu();
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    loop {
        if !model_setup_active() {
            print!("> ");
            let _ = io::stdout().flush();
        }
        let mut line = String::new();
        let n = reader
            .read_line(&mut line)
            .map_err(|e| e.to_string())?;
        if n == 0 {
            break;
        }
        let input = line.trim();
        if model_setup_active() {
            control_feed_setup(input);
            continue;
        }
        if !control_menu_command(input) {
            break;
        }
    }
    control_shutdown();
    Ok(())
}

fn control_menu_command(input: &str) -> bool {
    match input {
        "a" => control_start_server(),
        "b" => control_stop_server_async(),
        "c" => control_begin_setup(),
        "s" => control_show_status(),
        "m" => view_show_menu(),
        "k" => view_clear_screen(),
        "x" => return false,
        "" => {}
        other => view_unknown_command(other),
    }
    true
}

pub fn control_show_status() {
    thread::spawn(|| {
        let cfg = model_config_snapshot();
        let running = model_running();
        let conns = model_conn_count();
        let events = model_events_snapshot();
        view_show_status(running, &cfg, conns, &events);
    });
}