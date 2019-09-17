use ring::{
    rand,
    signature::{self,RsaKeyPair}
};

pub struct PublicKeyByte(&'static Vec<u8>);

impl PublicKeyByte {
    pub fn as_u8(&self) -> &'static Vec<u8> {
        self.0
    }
}

pub struct PrivateKeyByte(&'static Vec<u8>);

impl PrivateKeyByte {
    pub fn as_u8(&self) -> &'static Vec<u8> {
        self.0
    }
}

pub struct KeyPair{
    pub public_key_byte: &'static PublicKeyByte,
    pub private_key_byte: &'static PrivateKeyByte
}

#[derive(Debug)]
struct KeyPairGenerator {}

impl KeyPairGenerator {
    fn generate() {
        RsaKeyPair::from_pkcs8(&vec![]);
    }
}