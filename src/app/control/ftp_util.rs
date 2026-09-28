// src/app/control/ftp_util.rs

use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::path::{Path, PathBuf};

pub fn ftp_reply(
    w: &mut TcpStream,
    code: u16,
    text: &str,
) -> io::Result<()> {
    let line = format!("{} {}\r\n", code, text);
    w.write_all(line.as_bytes())
}

pub fn ftp_split_command(line: &str) -> (String, String) {
    match line.find(' ') {
        Some(i) => (
            line[..i].to_uppercase(),
            line[i + 1..].trim().to_string(),
        ),
        None => (line.to_uppercase(), String::new()),
    }
}

pub fn ftp_resolve_virtual(cwd: &str, arg: &str) -> String {
    let base = if arg.starts_with('/') {
        String::from(arg)
    } else {
        format!("{}/{}", cwd, arg)
    };
    ftp_normalize_virtual(&base)
}

fn ftp_normalize_virtual(base: &str) -> String {
    let mut parts: Vec<&str> = Vec::new();
    for part in base.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            p if ftp_is_unsafe_part(p) => {}
            p => parts.push(p),
        }
    }
    format!("/{}", parts.join("/"))
}

fn ftp_is_unsafe_part(p: &str) -> bool {
    p.contains(':')
        || p.contains('\\')
        || p.contains('\0')
}

pub fn ftp_resolve_path(
    root: &Path,
    vpath: &str,
) -> PathBuf {
    let rel = vpath.trim_start_matches('/');
    root.join(rel)
}

pub fn ftp_strip_list_flags(arg: &str) -> String {
    let mut clean = String::new();
    for part in arg.split_whitespace() {
        if part.starts_with('-') {
            continue;
        }
        if !clean.is_empty() {
            clean.push(' ');
        }
        clean.push_str(part);
    }
    clean
}

pub fn ftp_passive_ip_bytes(ip: IpAddr) -> [u8; 4] {
    match ip {
        IpAddr::V4(v4) => v4.octets(),
        IpAddr::V6(_) => [127, 0, 0, 1],
    }
}