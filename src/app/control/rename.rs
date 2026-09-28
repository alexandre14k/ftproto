// src/app/control/rename.rs

use std::fs;
use std::io;
use std::net::TcpStream;

use crate::app::*;

pub fn ftp_cmd_dele(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    if arg.is_empty() {
        return ftp_reply(w, 501, "missing path");
    }
    let target = ftp_resolve_virtual(&s.cwd, arg);
    let path = ftp_resolve_path(&s.root, &target);
    match fs::remove_file(&path) {
        Ok(_) => {
            model_send_event(&format!(
                "conn #{} deleted {}",
                s.conn_id, target
            ));
            ftp_reply(w, 250, "file deleted")
        }
        Err(_) => ftp_reply(w, 550, "cannot delete"),
    }
}

pub fn ftp_cmd_rnfr(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    if arg.is_empty() {
        return ftp_reply(w, 501, "missing path");
    }
    s.rename_from = Some(ftp_resolve_virtual(&s.cwd, arg));
    ftp_reply(w, 350, "ready for destination")
}

pub fn ftp_cmd_rnto(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    if arg.is_empty() {
        return ftp_reply(w, 501, "missing path");
    }
    let from = match s.rename_from.take() {
        Some(f) => f,
        None => {
            return ftp_reply(w, 503, "no RNFR given")
        }
    };
    let to = ftp_resolve_virtual(&s.cwd, arg);
    let src = ftp_resolve_path(&s.root, &from);
    let dst = ftp_resolve_path(&s.root, &to);
    match fs::rename(&src, &dst) {
        Ok(_) => {
            model_send_event(&format!(
                "conn #{} renamed {} to {}",
                s.conn_id, from, to
            ));
            ftp_reply(w, 250, "renamed")
        }
        Err(_) => ftp_reply(w, 550, "cannot rename"),
    }
}