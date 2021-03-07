use std::convert::TryInto;

use sha2::{Sha256, Digest};
use ripemd160::{Ripemd160};
use secp256k1::{Secp256k1, SecretKey, PublicKey};

pub struct Address
{
    pub private_key: [u8; 32],
    pub public_key: [u8; 64],
}

impl Address
{
    pub fn new(private_key_str: &[u8; 32]) -> Self
    {
        let secp = Secp256k1::new();

        let private_key = SecretKey::from_slice(private_key_str).expect("32 bytes, within curve order");
        let public_key = PublicKey::from_secret_key(&secp, &private_key);

        let mut private_key_out: [u8; 32];
        let mut public_key_out: [u8; 64];

        private_key_out = [0u8; 32];
        public_key_out = [0u8; 64];

        private_key_out[..].clone_from_slice(&private_key[..32]);
        public_key_out[..].clone_from_slice(&public_key.serialize_uncompressed()[1..65]);

        Self
        {
            private_key: private_key_out,
            public_key: public_key_out,
        }
    }

    pub fn get(&self) -> [u8; 20]
    {
        // SHA256 the public key
        let sha2: [u8; 32] = Sha256::new()
            .chain(&self.public_key)
            .finalize()
            .as_slice()
            .try_into()
            .expect("Invalid length");

        // RIPEMD160 the SHA'd public key to get
        // the 20 byte address
        Ripemd160::new()
            .chain(sha2)
            .finalize()
            .as_slice()
            .try_into()
            .expect("Invalid length")
    }
}
