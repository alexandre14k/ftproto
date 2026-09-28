// src/app/control/dataconn.rs

use std::io;
use std::net::{
    IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream,
};

use crate::app::*;

pub enum FtpData {
    Passive(TcpListener),
    Active(SocketAddr),
}

pub fn ftp_cmd_pasv(
    s: &mut FtpSession,
    w: &mut TcpStream,
) -> io::Result<()> {
    let port = match data_passive_port(s) {
        Some(p) => p,
        None => {
            return ftp_reply(w, 425, "cannot open port")
        }
    };
    let ip = ftp_passive_ip_bytes(s.local_ip);
    let text = format!(
        "Entering Passive Mode ({},{},{},{},{},{})",
        ip[0], ip[1], ip[2], ip[3],
        port / 256, port % 256,
    );
    ftp_reply(w, 227, &text)
}

pub fn ftp_cmd_epsv(
    s: &mut FtpSession,
    w: &mut TcpStream,
) -> io::Result<()> {
    let port = match data_passive_port(s) {
        Some(p) => p,
        None => {
            return ftp_reply(w, 425, "cannot open port")
        }
    };
    let text = format!(
        "Entering Extended Passive Mode (|||{}|)",
        port
    );
    ftp_reply(w, 229, &text)
}

pub fn ftp_cmd_port(
    s: &mut FtpSession,
    w: &mut TcpStream,
    arg: &str,
) -> io::Result<()> {
    let nums: Vec<u16> = arg
        .split(',')
        .map(|p| p.trim().parse::<u16>())
        .filter_map(|r| r.ok())
        .collect();
    if nums.len() != 6 {
        return ftp_reply(w, 501, "bad PORT syntax");
    }
    let ip = IpAddr::V4(Ipv4Addr::new(
        nums[0] as u8,
        nums[1] as u8,
        nums[2] as u8,
        nums[3] as u8,
    ));
    let port = nums[4] * 256 + nums[5];
    s.data =
        Some(FtpData::Active(SocketAddr::new(ip, port)));
    ftp_reply(w, 200, "active mode set")
}

fn data_passive_port(s: &mut FtpSession) -> Option<u16> {
    let listener = data_bind_passive()?;
    let port = listener.local_addr().map(|a| a.port()).ok()?;
    s.data = Some(FtpData::Passive(listener));
    Some(port)
}

fn data_bind_passive() -> Option<TcpListener> {
    let l = TcpListener::bind("0.0.0.0:0").ok()?;
    let _ = l.set_nonblocking(true);
    Some(l)
}