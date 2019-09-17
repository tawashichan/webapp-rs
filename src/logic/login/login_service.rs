use crate::logic::login::jwt_handler::{TJWTHandler};
use crate::logic::login::token::{TokenString};

// 型パラメータまみれになるのを防ぐためにtrait objectを使ったが,動的ヂスパッチになりパフォーマンス上の問題があるためproductionで使って良いか微妙
pub struct LoginService {
    jwt_handler: Box<dyn TJWTHandler> //JWTHandler
}

impl LoginService {

    pub fn new(jwt_handler: Box<dyn TJWTHandler>) -> Self {
        LoginService{
            jwt_handler,
        }
    }

    fn login(&self) -> Result<TokenString,()>{
        self.jwt_handler.generate()
    }
}