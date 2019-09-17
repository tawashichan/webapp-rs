use ring::{
    rand,
    signature::{self,RsaKeyPair}
};

#[derive(Debug)]
struct KeyPairGenerator {}

impl KeyPairGenerator {
    fn generate() {
        RsaKeyPair::from_pkcs8(&vec![]);
    }
}