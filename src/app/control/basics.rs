// src/app/control/basics.rs

use std::io::{self, Write};
use std::net::TcpStream;

use crate::app::*;

pub fn ftp_cmd_user(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    s.pending_user = Some(String::from(arg));
    ftp_reply(w, 331, "password required")
}

pub fn ftp_cmd_pass(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    let name = match s.pending_user.clone() {
        Some(n) => n,
        None => String::new(),
    };
    if name == "ftproto" && arg == "ftproto" {
        s.logged_in = true;
        model_send_event(&format!(
            "conn #{} login ftproto",
            s.conn_id
        ));
        ftp_reply(w, 230, "logged in")
    } else {
        s.logged_in = false;
        model_send_event(&format!(
            "conn #{} login failed for {}",
            s.conn_id, name
        ));
        ftp_reply(w, 530, "login incorrect")
    }
}

pub fn ftp_cmd_feat(w: &mut TcpStream) -> io::Result<()> {
    let feats = [
        "EPSV",
        "MDTM",
        "PASV",
        "REST STREAM",
        "SIZE",
        "UTF8",
    ];
    let mut out = String::from("211-Features:\r\n");
    for f in feats {
        out.push(' ');
        out.push_str(f);
        out.push_str("\r\n");
    }
    out.push_str("211 End\r\n");
    w.write_all(out.as_bytes())
}

pub fn ftp_cmd_pwd(
    s: &FtpSession,
    w: &mut TcpStream,
) -> io::Result<()> {
    let text = format!(
        "\"{}\" is current directory",
        s.cwd
    );
    ftp_reply(w, 257, &text)
}

pub fn ftp_cmd_cdup(
    s: &mut FtpSession,
    w: &mut TcpStream,
) -> io::Result<()> {
    s.cwd = ftp_resolve_virtual(&s.cwd, "..");
    ftp_reply(w, 250, "directory changed")
}

pub fn ftp_cmd_rest(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    match arg.parse::<u64>() {
        Ok(n) => {
            s.rest_offset = n;
            ftp_reply(w, 350, "restart point set")
        }
        Err(_) => ftp_reply(w, 501, "bad offset"),
    }
}