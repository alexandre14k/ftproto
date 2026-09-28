// src/app/control/session_io.rs

use std::io;
use std::net::TcpStream;
use std::time::Duration;

use crate::app::*;

const SESSION_POLL_MS: u64 = 200;
const SESSION_WRITE_SECS: u64 = 30;

pub fn session_tune_stream(stream: &mut TcpStream) {
    let _ = stream.set_read_timeout(Some(
        Duration::from_millis(SESSION_POLL_MS),
    ));
    let _ = stream.set_write_timeout(Some(
        Duration::from_secs(SESSION_WRITE_SECS),
    ));
}

pub fn session_retry(e: &io::Error) -> bool {
    matches!(
        e.kind(),
        io::ErrorKind::WouldBlock
            | io::ErrorKind::TimedOut
            | io::ErrorKind::Interrupted
    )
}

pub fn session_consume(
    sess: &mut FtpSession,
    stream: &mut TcpStream,
    pending: &mut String,
) -> bool {
    while let Some(pos) = pending.find('\n') {
        let line = pending[..pos]
            .trim_end_matches('\r')
            .to_string();
        pending.drain(..pos + 1);
        if line.is_empty() {
            continue;
        }
        let (cmd, arg) = ftp_split_command(&line);
        if control_ftp_dispatch(sess, stream, &cmd, &arg) {
            return true;
        }
    }
    false
}