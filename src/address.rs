use sha2::{Sha256, Digest};
use ripemd160::{Ripemd160};
use secp256k1::{Secp256k1, SecretKey, PublicKey};

pub struct Address
{
    private_key: [u8; 32],
    public_key: [u8; 64],
}

impl Address
{
    pub fn new() -> Self
    {
        let secp = Secp256k1::new();

        let private_key_str = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
        let public_key_str = PublicKey::from_secret_key(&secp, &private_key_str);

        println!("{:?}", private_key_str);
        let mut private_key: [u8; 32];
        let mut public_key: [u8; 64];

        private_key = [0u8; 32];
        public_key = [0u8; 64];

        //private_key[..].clone_from_slice(&private_key_str[..32]);
        //public_key[..].clone_from_slice(&public_key_str.serialize_uncompressed()[1..65]);
        
        Self
        {
            private_key,
            public_key,
        }
    }

    pub fn get(&self) -> [u8; 20]
    {
        let sha2_hash = Sha256::digest(&self.public_key);
        let mut ripemd_hasher = Ripemd160::new();

        // Create the address from the public key
        ripemd_hasher.update(sha2_hash.as_slice());

        ripemd_hasher.finalize().as_slice().try_into().expect("Invalid length")
    }
}
