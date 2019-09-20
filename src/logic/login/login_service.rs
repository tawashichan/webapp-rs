use crate::logic::login::jwt_handler::{TJWTHandler};
use crate::logic::login::token::{TokenString};

pub struct LoginService<'a> {
    jwt_handler: &'a dyn TJWTHandler
}

impl <'a>LoginService<'a> {

    pub fn new(jwt_handler: &'a dyn TJWTHandler) -> Self {
        LoginService{
            jwt_handler,
        }
    }

    fn login(&self) -> Result<TokenString,()>{
        self.jwt_handler.generate()
    }
}