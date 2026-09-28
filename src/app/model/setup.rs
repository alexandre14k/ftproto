// src/app/model/setup.rs

use super::state::model_lock;

#[derive(Clone, Copy)]
pub enum SetupStage {
    Port,
    Ip,
    Folder,
    Confirm,
}

pub struct SetupFlow {
    pub stage: SetupStage,
    pub listen_ip: String,
    pub listen_port: u16,
    pub files_root: String,
}

pub fn model_setup_active() -> bool {
    model_lock().lock().unwrap().setup.is_some()
}

pub fn model_set_setup(f: SetupFlow) {
    model_lock().lock().unwrap().setup = Some(f);
}

pub fn model_take_setup() -> Option<SetupFlow> {
    model_lock().lock().unwrap().setup.take()
}

pub fn model_update_setup<R>(
    f: impl FnOnce(&mut SetupFlow) -> R,
) -> Option<R> {
    let mut m = model_lock().lock().unwrap();
    m.setup.as_mut().map(f)
}