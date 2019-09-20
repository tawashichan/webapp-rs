

mod infra;
mod logic;
mod web;
mod initializer;

use crate::logic::login::login_service::{LoginService};
use crate::infra::jwt_handler::{JWTHandler};

fn main() {
    let jwt_handler: JWTHandler = JWTHandler::new(vec![]);
    let login_service = LoginService::new(&jwt_handler);
    println!("Hello, world!");
}