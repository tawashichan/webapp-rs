use crate::logic::login::token::{TokenString};

pub trait TJWTHandler {
    fn generate(&self) -> Result<TokenString,()>;
}