// src/app/control/stream_io.rs

use std::io::{self, Read, Write};
use std::sync::atomic::Ordering;
use std::time::Duration;

use crate::app::*;

const DATA_WAIT_SECS: u64 = 30;

pub fn ftp_open_data(s: &mut FtpSession) -> DataConn {
    let wait = Duration::from_secs(DATA_WAIT_SECS);
    match s.data.take() {
        Some(FtpData::Passive(l)) => {
            stream_accept_timeout(&l, wait, &s.stop)
        }
        Some(FtpData::Active(a)) => {
            stream_connect_timeout(a, wait)
        }
        None => None,
    }
}

pub fn stream_copy_file(
    from: &mut dyn Read,
    to: &mut dyn Write,
    stop: &StopFlag,
) -> io::Result<u64> {
    let mut buf = [0u8; 8192];
    let mut total: u64 = 0;
    loop {
        if stop.load(Ordering::Relaxed) {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "server stopped",
            ));
        }
        let n = from.read(&mut buf)?;
        if n == 0 {
            return Ok(total);
        }
        to.write_all(&buf[..n])?;
        total += n as u64;
    }
}