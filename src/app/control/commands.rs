// src/app/control/commands.rs

use std::net::TcpStream;

use crate::app::*;

pub fn control_ftp_dispatch(
    s: &mut FtpSession,
    w: &mut TcpStream,
    cmd: &str,
    arg: &str,
) -> bool {
    match cmd {
        "QUIT" => {
            let _ = ftp_reply(w, 221, "bye");
            return true;
        }
        "USER" => { let _ = ftp_cmd_user(s, w, arg); }
        "PASS" => { let _ = ftp_cmd_pass(s, w, arg); }
        "NOOP" | "OPTS" | "TYPE" | "ALLO" => {
            let _ = ftp_reply(w, 200, "ok");
        }
        "SYST" => {
            let _ = ftp_reply(w, 215, "UNIX Type: L8");
        }
        "FEAT" => { let _ = ftp_cmd_feat(w); }
        "MODE" | "STRU" => {
            let _ = ftp_reply(w, 200, "ok");
        }
        _ if !s.logged_in => {
            let _ = ftp_reply(w, 530, "please log in");
        }
        "PWD" | "XPWD" => { let _ = ftp_cmd_pwd(s, w); }
        "CWD" | "XCWD" => { let _ = ftp_cmd_cwd(s, w, arg); }
        "CDUP" | "XCUP" => { let _ = ftp_cmd_cdup(s, w); }
        "PASV" => { let _ = ftp_cmd_pasv(s, w); }
        "EPSV" => { let _ = ftp_cmd_epsv(s, w); }
        "PORT" => { let _ = ftp_cmd_port(s, w, arg); }
        "LIST" => { let _ = ftp_cmd_list(s, w, arg, true); }
        "NLST" => { let _ = ftp_cmd_list(s, w, arg, false); }
        "SIZE" => { let _ = ftp_cmd_size(s, w, arg); }
        "MDTM" => { let _ = ftp_cmd_mdtm(s, w, arg); }
        "MKD" | "XMKD" => { let _ = ftp_cmd_mkd(s, w, arg); }
        "RMD" | "XRMD" => { let _ = ftp_cmd_rmd(s, w, arg); }
        "DELE" => { let _ = ftp_cmd_dele(s, w, arg); }
        "RNFR" => { let _ = ftp_cmd_rnfr(s, w, arg); }
        "RNTO" => { let _ = ftp_cmd_rnto(s, w, arg); }
        "REST" => { let _ = ftp_cmd_rest(s, w, arg); }
        "RETR" => { let _ = ftp_cmd_retr(s, w, arg); }
        "STOR" => {
            let _ = ftp_cmd_stor(s, w, arg, false);
        }
        "APPE" => {
            let _ = ftp_cmd_stor(s, w, arg, true);
        }
        "ABOR" => {
            s.data = None;
            let _ = ftp_reply(w, 226, "aborted");
        }
        _ => {
            let _ = ftp_reply(w, 502, "not implemented");
        }
    }
    false
}