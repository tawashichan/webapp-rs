use crate::logic::login::login_service::{LoginService};
use crate::infra::jwt_handler::{JWTHandler};

pub struct Services {
    pub login: LoginService
}

pub fn init_services() -> Services {
    let jwt_handler = JWTHandler::new(vec![]);

    let login_service = LoginService::new(Box::new(jwt_handler));
    Services{
        login: login_service
    }
}