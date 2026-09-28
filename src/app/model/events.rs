// src/app/model/events.rs

use std::sync::mpsc::TryRecvError;

use super::state::model_lock;

const EVENT_CAP: usize = 1000;

pub fn model_send_event(text: &str) {
    let m = model_lock().lock().unwrap();
    let _ = m.event_tx.send(String::from(text));
}

pub fn model_collect_events() {
    let mut m = model_lock().lock().unwrap();
    loop {
        let next = {
            let rx = m.event_rx.lock().unwrap();
            rx.try_recv()
        };
        match next {
            Ok(text) => m.events.push(text),
            Err(TryRecvError::Empty) => break,
            Err(TryRecvError::Disconnected) => break,
        }
    }
    if m.events.len() > EVENT_CAP {
        let extra = m.events.len() - EVENT_CAP;
        m.events.drain(0..extra);
    }
}

pub fn model_events_snapshot() -> Vec<String> {
    model_collect_events();
    model_lock().lock().unwrap().events.clone()
}