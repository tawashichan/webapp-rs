
use frank_jwt::{encode,Algorithm};
use serde_json::{json};

use crate::logic::login::token::{TokenString};
use crate::logic::login::jwt_handler::{TJWTHandler};

pub struct JWTHandler {
    pub_key_bytes: Vec<Vec<u8>>
}

impl JWTHandler {
    pub fn new(pub_key_bytes: Vec<Vec<u8>>) -> Self {
        JWTHandler{
            pub_key_bytes,
        }
    }
}

impl TJWTHandler for JWTHandler {
    fn generate(&self) -> Result<TokenString,()>{ 
        let header = json!({});
        let payload = json!({});
        let token = encode(header, &self.pub_key_bytes[0],&payload,Algorithm::RS256).unwrap();
        Ok(TokenString(token))
    }
}