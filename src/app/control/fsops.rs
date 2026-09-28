// src/app/control/fsops.rs

use std::fs;
use std::io;
use std::net::TcpStream;

use crate::app::*;

pub fn ftp_cmd_cwd(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    if arg.is_empty() {
        return ftp_reply(w, 501, "missing path");
    }
    let target = ftp_resolve_virtual(&s.cwd, arg);
    let path = ftp_resolve_path(&s.root, &target);
    match fs::metadata(&path) {
        Ok(m) if m.is_dir() => {
            s.cwd = target;
            ftp_reply(w, 250, "directory changed")
        }
        _ => ftp_reply(w, 550, "directory not found"),
    }
}

pub fn ftp_cmd_mkd(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    if arg.is_empty() {
        return ftp_reply(w, 501, "missing path");
    }
    let target = ftp_resolve_virtual(&s.cwd, arg);
    let path = ftp_resolve_path(&s.root, &target);
    match fs::create_dir(&path) {
        Ok(_) => {
            model_send_event(&format!(
                "conn #{} made dir {}",
                s.conn_id, target
            ));
            ftp_reply(w, 257, "directory created")
        }
        Err(_) => ftp_reply(w, 550, "cannot create"),
    }
}

pub fn ftp_cmd_rmd(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    if arg.is_empty() {
        return ftp_reply(w, 501, "missing path");
    }
    let target = ftp_resolve_virtual(&s.cwd, arg);
    let path = ftp_resolve_path(&s.root, &target);
    match fs::remove_dir_all(&path) {
        Ok(_) => {
            model_send_event(&format!(
                "conn #{} removed dir {}",
                s.conn_id, target
            ));
            ftp_reply(w, 250, "directory removed")
        }
        Err(_) => ftp_reply(w, 550, "cannot remove"),
    }
}