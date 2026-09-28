// src/main.rs
#![windows_subsystem = "windows"]

mod app;

use app::*;

fn main() -> Result<(), String> {
    control_main()
}