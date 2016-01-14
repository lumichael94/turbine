extern crate rusqlite;
extern crate rustc_serialize;
extern crate bincode;

pub mod data;
pub mod engine;
pub mod util;
pub mod network;

use engine::turbo;

pub fn main() {
    turbo::init();
    turbo::command_loop();
}
