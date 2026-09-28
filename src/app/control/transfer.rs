// src/app/control/transfer.rs

use std::fs::File;
use std::io::{self, Seek, SeekFrom};
use std::net::{Shutdown, TcpStream};

use crate::app::*;

pub fn ftp_cmd_retr(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    if arg.is_empty() {
        return ftp_reply(w, 501, "missing path");
    }
    let target = ftp_resolve_virtual(&s.cwd, arg);
    let path = ftp_resolve_path(&s.root, &target);
    if !path.is_file() {
        return ftp_reply(w, 550, "file not found");
    }
    ftp_reply(w, 150, "opening data connection")?;
    let mut data = match ftp_open_data(s) {
        Some(d) => d,
        None => {
            return ftp_reply(w, 425, "no data connection")
        }
    };
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => {
            return ftp_reply(w, 451, "cannot open")
        }
    };
    if s.rest_offset > 0 {
        let _ = file.seek(SeekFrom::Start(s.rest_offset));
    }
    s.rest_offset = 0;
    match stream_copy_file(&mut file, &mut data, &s.stop) {
        Ok(n) => transfer_done(s, w, data, target, "retr", n),
        Err(_) => ftp_reply(w, 426, "transfer aborted"),
    }
}

pub fn transfer_done(
    s: &FtpSession,
    w: &mut TcpStream,
    data: TcpStream,
    target: String,
    verb: &str,
    n: u64,
) -> io::Result<()> {
    let _ = data.shutdown(Shutdown::Write);
    model_send_event(&format!(
        "conn #{} {} {} ({} bytes)",
        s.conn_id, verb, target, n
    ));
    ftp_reply(w, 226, "transfer complete")
}