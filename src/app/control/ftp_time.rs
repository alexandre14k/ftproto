// src/app/control/ftp_time.rs

use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn ftp_dir_line(
    name: &str,
    meta: &fs::Metadata,
) -> String {
    let kind = if meta.is_dir() { 'd' } else { '-' };
    let size = meta.len();
    let mtime = ftp_format_mtime(meta.modified().ok());
    format!(
        "{}rw-r--r-- 1 ftproto ftproto {:>10} {} {}",
        kind, size, mtime, name,
    )
}

fn ftp_format_mtime(st: Option<SystemTime>) -> String {
    let secs = match st
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
    {
        Some(d) => d.as_secs() as i64,
        None => 0,
    };
    let (_, m, d, hh, mm, _) = ftp_civil_time(secs);
    let months = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let mon = months[(m - 1) as usize];
    format!("{} {:02} {:02}:{:02}", mon, d, hh, mm)
}

pub fn ftp_format_stamp(t: SystemTime) -> String {
    let secs = t
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let (y, mo, d, hh, mm, ss) = ftp_civil_time(secs);
    format!(
        "{:04}{:02}{:02}{:02}{:02}{:02}",
        y, mo, d, hh, mm, ss
    )
}

fn ftp_civil_time(
    secs: i64,
) -> (i64, u32, u32, u32, u32, u32) {
    let days = secs.div_euclid(86400);
    let rem = secs.rem_euclid(86400);
    let hh = (rem / 3600) as u32;
    let mm = ((rem % 3600) / 60) as u32;
    let ss = (rem % 60) as u32;
    let (y, m, d) = ftp_civil_from_days(days);
    (y, m, d, hh, mm, ss)
}

fn ftp_civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = z.div_euclid(146097);
    let doe = z.rem_euclid(146097);
    let yoe = (doe - doe / 1460 + doe / 36524
        - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}