// src/app/view/setup_prompt.rs

use std::io::{self, Write};

pub fn view_setup_intro() {
    println!("setup: blank input keeps current value");
}

pub fn view_setup_prompt_port(current: u16) {
    print!("listen port [{}]: ", current);
    view_setup_flush();
}

pub fn view_setup_prompt_ip(current: &str) {
    print!("listen ip [{}]: ", current);
    view_setup_flush();
}

pub fn view_setup_prompt_root(current: &str) {
    print!("files folder [{}]: ", current);
    view_setup_flush();
}

pub fn view_setup_confirm(ip: &str, port: u16, root: &str) {
    println!("new settings:");
    println!("listen ip    : {}", ip);
    println!("listen port  : {}", port);
    println!("files folder : {}", root);
    print!("apply settings? (y/n): ");
    view_setup_flush();
}

pub fn view_setup_bad_port() {
    println!("invalid port, expected 1-65535");
}

pub fn view_setup_bad_ip() {
    println!("invalid ip address");
}

pub fn view_setup_bad_confirm() {
    println!("enter y or n");
}

pub fn view_setup_cancelled() {
    println!("setup cancelled");
}

fn view_setup_flush() {
    let _ = io::stdout().flush();
}