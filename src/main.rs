extern crate postgres;
extern crate rustc_serialize;
extern crate bincode;

pub mod data;
pub mod engine;
pub mod util;

use engine::turbo;

pub fn main() {
    turbo::init();
    turbo::command_loop();
}
