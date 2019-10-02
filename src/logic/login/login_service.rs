use crate::logic::login::jwt_handler::{TJWTHandler};
use crate::logic::login::token::{TokenString};
use crate::logic::repository::user::TUserRepository;

pub struct LoginService<'a> {
    jwt_handler: &'a dyn TJWTHandler,
    user_repository: &'a dyn TUserRepository
}

impl <'a>LoginService<'a> {

    pub fn new(
        jwt_handler: &'a dyn TJWTHandler,
        user_repository: &'a dyn TUserRepository
        ) -> Self {
        LoginService{
            jwt_handler,
            user_repository,
        }
    }

    pub fn login(&self) /*-> Result<TokenString,()>*/{
        let user = self.user_repository.find_by_id("1".to_string());
        //self.jwt_handler.generate()
    }


}