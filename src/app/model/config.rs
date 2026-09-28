// src/app/model/config.rs

pub struct FtpConfig {
    pub listen_ip: String,
    pub listen_port: u16,
    pub files_root: String,
}

impl FtpConfig {
    pub fn ftp_default_config() -> FtpConfig {
        FtpConfig {
            listen_ip: String::from("0.0.0.0"),
            listen_port: 2121,
            files_root: String::from("files"),
        }
    }
}