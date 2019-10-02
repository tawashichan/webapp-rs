
mod infra;
mod logic;
mod web;
mod initializer;
mod table;
mod schema;

#[macro_use]
extern crate diesel;

use crate::logic::login::login_service::{LoginService};
use crate::infra::jwt_handler::{JWTHandler};
use crate::infra::db::connection;
use crate::infra::db::user_repository;

fn main() {
    let pool = connection::establish_connection();
    let user_repository = user_repository::UserRepository::new(pool);

    let jwt_handler: JWTHandler = JWTHandler::new(vec![]);
    let login_service = LoginService::new(&jwt_handler,&user_repository);
    login_service.login();
    println!("Hello, world!");
}