// src/app/model/mod.rs

mod config;
mod events;
mod runtime;
mod setup;
mod state;

pub use config::*;
pub use events::*;
pub use runtime::*;
pub use setup::*;
pub use state::*;

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

pub type StopFlag = Arc<AtomicBool>;
pub type ConnRegistry = Arc<Mutex<Vec<JoinHandle<()>>>>;