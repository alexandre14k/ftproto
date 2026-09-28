// src/app/control/store.rs

use std::fs::OpenOptions;
use std::io;
use std::net::TcpStream;

use crate::app::*;

pub fn ftp_cmd_stor(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
    append: bool,
) -> io::Result<()> {
    if arg.is_empty() {
        return ftp_reply(w, 501, "missing path");
    }
    let target = ftp_resolve_virtual(&s.cwd, arg);
    let path = ftp_resolve_path(&s.root, &target);
    ftp_reply(w, 150, "opening data connection")?;
    let mut data = match ftp_open_data(s) {
        Some(d) => d,
        None => {
            return ftp_reply(w, 425, "no data connection")
        }
    };
    let mut opts = OpenOptions::new();
    opts.create(true);
    if append {
        opts.append(true);
    } else {
        opts.write(true).truncate(true);
    }
    let mut file = match opts.open(&path) {
        Ok(f) => f,
        Err(_) => {
            return ftp_reply(w, 451, "cannot open")
        }
    };
    let verb = if append { "appe" } else { "stor" };
    match stream_copy_file(&mut data, &mut file, &s.stop) {
        Ok(n) => transfer_done(s, w, data, target, verb, n),
        Err(_) => ftp_reply(w, 426, "transfer aborted"),
    }
}