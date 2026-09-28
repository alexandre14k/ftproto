// src/app/view/menu.rs

use std::io::{self, Write};

pub fn view_show_menu() {
    let lines = [
        "# ftproto is an FTP server",
        "a -- start server",
        "b -- stop server",
        "c -- setup server",
        "s -- show status",
        "m -- show menu",
        "k -- clear screen",
        "x -- exit",
    ];
    for line in lines {
        println!("{}", line);
    }
}

pub fn view_unknown_command(cmd: &str) {
    println!("unknown command: {} (m shows menu)", cmd);
}

pub fn view_clear_screen() {
    print!("\x1b[2J\x1b[1;1H");
    let _ = io::stdout().flush();
}