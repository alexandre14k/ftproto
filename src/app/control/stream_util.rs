// src/app/control/stream_util.rs

use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::atomic::Ordering;
use std::thread;
use std::time::{Duration, Instant};

use crate::app::*;

const DATA_IO_SECS: u64 = 30;
const POLL_MS: u64 = 20;

pub fn stream_accept_timeout(
    l: &TcpListener,
    wait: Duration,
    stop: &StopFlag,
) -> Option<TcpStream> {
    let deadline = Instant::now() + wait;
    loop {
        if stop.load(Ordering::Relaxed) {
            return None;
        }
        match l.accept() {
            Ok((s, _)) => {
                stream_tune_data(&s);
                return Some(s);
            }
            Err(_) => {
                if Instant::now() >= deadline {
                    return None;
                }
                thread::sleep(Duration::from_millis(POLL_MS));
            }
        }
    }
}

pub fn stream_connect_timeout(
    addr: SocketAddr,
    wait: Duration,
) -> Option<TcpStream> {
    match TcpStream::connect_timeout(&addr, wait) {
        Ok(s) => {
            stream_tune_data(&s);
            Some(s)
        }
        Err(_) => None,
    }
}

fn stream_tune_data(s: &TcpStream) {
    let t = Some(Duration::from_secs(DATA_IO_SECS));
    let _ = s.set_read_timeout(t);
    let _ = s.set_write_timeout(t);
}