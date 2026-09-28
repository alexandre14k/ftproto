// src/app/control/setup_advance.rs

use std::net::IpAddr;

use crate::app::*;

pub fn setup_advance(
    f: &mut SetupFlow,
    input: &str,
) -> SetupOutcome {
    match f.stage {
        SetupStage::Port => setup_advance_port(f, input),
        SetupStage::Ip => setup_advance_ip(f, input),
        SetupStage::Folder => {
            setup_advance_folder(f, input)
        }
        SetupStage::Confirm => {
            setup_advance_confirm(f, input)
        }
    }
}

fn setup_advance_port(
    f: &mut SetupFlow,
    input: &str,
) -> SetupOutcome {
    let mut ok = input.is_empty();
    if !ok {
        if let Ok(p) = input.parse::<u16>() {
            if p >= 1 {
                f.listen_port = p;
                ok = true;
            }
        }
    }
    if ok {
        f.stage = SetupStage::Ip;
        view_setup_prompt_ip(&f.listen_ip);
    } else {
        view_setup_bad_port();
    }
    SetupOutcome::Pending
}

fn setup_advance_ip(
    f: &mut SetupFlow,
    input: &str,
) -> SetupOutcome {
    let mut ok = input.is_empty();
    if !ok && input.parse::<IpAddr>().is_ok() {
        f.listen_ip = String::from(input);
        ok = true;
    }
    if ok {
        f.stage = SetupStage::Folder;
        view_setup_prompt_root(&f.files_root);
    } else {
        view_setup_bad_ip();
    }
    SetupOutcome::Pending
}

fn setup_advance_folder(
    f: &mut SetupFlow,
    input: &str,
) -> SetupOutcome {
    if !input.is_empty() {
        f.files_root = String::from(input);
    }
    f.stage = SetupStage::Confirm;
    view_setup_confirm(
        &f.listen_ip,
        f.listen_port,
        &f.files_root,
    );
    SetupOutcome::Pending
}

fn setup_advance_confirm(
    f: &mut SetupFlow,
    input: &str,
) -> SetupOutcome {
    match input.to_lowercase().as_str() {
        "y" | "yes" => SetupOutcome::Apply(FtpConfig {
            listen_ip: f.listen_ip.clone(),
            listen_port: f.listen_port,
            files_root: f.files_root.clone(),
        }),
        "n" | "no" => SetupOutcome::Cancel,
        _ => {
            view_setup_bad_confirm();
            SetupOutcome::Pending
        }
    }
}