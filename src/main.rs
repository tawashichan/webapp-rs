

mod infra;
mod logic;
mod web;
mod initializer;

use crate::web::init::init::init;

fn main() {
    init();
    println!("Hello, world!");
}
