// src/app/control/session.rs

use std::io::Read;
use std::net::{IpAddr, TcpStream};
use std::path::PathBuf;
use std::sync::atomic::Ordering;

use crate::app::*;

pub struct FtpSession {
    pub conn_id: u32,
    pub root: PathBuf,
    pub local_ip: IpAddr,
    pub cwd: String,
    pub pending_user: Option<String>,
    pub logged_in: bool,
    pub rename_from: Option<String>,
    pub rest_offset: u64,
    pub data: Option<FtpData>,
    pub stop: StopFlag,
}

pub fn control_session_run(
    conn_id: u32,
    mut stream: TcpStream,
    root: PathBuf,
    stop: StopFlag,
) {
    session_tune_stream(&mut stream);
    let local_ip = stream
        .local_addr()
        .map(|a| a.ip())
        .unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
    let mut sess = FtpSession {
        conn_id,
        root,
        local_ip,
        cwd: String::from("/"),
        pending_user: None,
        logged_in: false,
        rename_from: None,
        rest_offset: 0,
        data: None,
        stop,
    };
    if ftp_reply(&mut stream, 220, "ftproto ready").is_err() {
        return;
    }
    let mut pending = String::new();
    let mut buf = [0u8; 1024];
    loop {
        if sess.stop.load(Ordering::Relaxed) {
            break;
        }
        let n = match stream.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => n,
            Err(e) if session_retry(&e) => continue,
            Err(_) => break,
        };
        pending.push_str(&String::from_utf8_lossy(&buf[..n]));
        if session_consume(
            &mut sess,
            &mut stream,
            &mut pending,
        ) {
            break;
        }
    }
    model_send_event(&format!("conn #{} closed", conn_id));
}