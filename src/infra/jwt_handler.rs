
use frank_jwt::{encode,decode,Algorithm};
use serde_json::{json};

use crate::logic::login::token::{TokenString};
use crate::infra::key_pair_generator::{KeyPair};
use crate::logic::login::jwt_handler::{TJWTHandler};

pub struct JWTHandler {
    key_pairs: Vec<KeyPair>
}

impl JWTHandler {
    pub fn new(key_pairs: Vec<KeyPair>) -> Self {
        JWTHandler{
            key_pairs,
        }
    }
}

impl TJWTHandler for JWTHandler {
    fn generate(&self) -> Result<TokenString,()>{ 
        let header = json!({});
        let payload = json!({});
        let token = encode(header, self.key_pairs[0].private_key_byte.as_u8(),&payload,Algorithm::RS256).unwrap();
        Ok(TokenString(token))
    }
}