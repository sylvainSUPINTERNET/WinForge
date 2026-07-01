use std::sync::OnceLock;
use crossbeam_channel::Sender;

pub static TX: OnceLock<Sender<String>> = OnceLock::new();