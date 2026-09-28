// src/app/control/listing.rs

use std::fs;
use std::io::{self, Write};
use std::net::{Shutdown, TcpStream};

use crate::app::*;

pub fn ftp_cmd_list(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
    detailed: bool,
) -> io::Result<()> {
    let clean = ftp_strip_list_flags(arg);
    let target = ftp_resolve_virtual(&s.cwd, &clean);
    let path = ftp_resolve_path(&s.root, &target);
    let is_dir = fs::metadata(&path)
        .map(|m| m.is_dir())
        .unwrap_or(false);
    if !is_dir {
        return ftp_reply(w, 550, "no such directory");
    }
    ftp_reply(w, 150, "opening data connection")?;
    let mut data = match ftp_open_data(s) {
        Some(d) => d,
        None => {
            return ftp_reply(w, 425, "no data connection")
        }
    };
    let entries = match fs::read_dir(&path) {
        Ok(rd) => rd,
        Err(_) => {
            let _ = data
                .shutdown(std::net::Shutdown::Write);
            return ftp_reply(w, 451, "cannot read");
        }
    };
    let mut body = String::new();
    for entry in entries.flatten() {
        let name =
            entry.file_name().to_string_lossy().to_string();
        let meta = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };
        if detailed {
            body.push_str(&ftp_dir_line(&name, &meta));
        } else {
            body.push_str(&name);
        }
        body.push_str("\r\n");
    }
    let res = data.write_all(body.as_bytes());
    let _ = data.shutdown(Shutdown::Write);
    if res.is_err() {
        return ftp_reply(w, 426, "transfer aborted");
    }
    model_send_event(&format!(
        "conn #{} list {}",
        s.conn_id, target
    ));
    ftp_reply(w, 226, "transfer complete")
}

pub fn ftp_cmd_size(
    s: &FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    if arg.is_empty() {
        return ftp_reply(w, 501, "missing path");
    }
    let path = ftp_resolve_path(
        &s.root,
        &ftp_resolve_virtual(&s.cwd, arg),
    );
    match fs::metadata(&path) {
        Ok(m) if m.is_file() => {
            ftp_reply(w, 213, &m.len().to_string())
        }
        _ => ftp_reply(w, 550, "file not found"),
    }
}

pub fn ftp_cmd_mdtm(
    s: &FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    if arg.is_empty() {
        return ftp_reply(w, 501, "missing path");
    }
    let path = ftp_resolve_path(
        &s.root,
        &ftp_resolve_virtual(&s.cwd, arg),
    );
    let modified = fs::metadata(&path).and_then(|m| {
        m.modified()
    });
    match modified {
        Ok(t) => {
            let text = ftp_format_stamp(t);
            ftp_reply(w, 213, &text)
        }
        Err(_) => ftp_reply(w, 550, "file not found"),
    }
}