use std::convert::TryInto;

use sha2::{Sha256, Digest};

use crate::transaction;

pub struct Block
{
    index: u32,
    previous_hash: [u8; 32],
    timestamp: u64,
    transactions: Vec<transaction::Transaction>,
    nonce: u32,
}

impl Block
{
    pub fn new(index: u32, previous_hash: [u8; 32], timestamp: u64, transactions: Vec<transaction::Transaction>) -> Self
    {
        let nonce: u32 = 0;

        Self
        {
            index,
            previous_hash,
            timestamp,
            transactions,
            nonce,
        }
    }

    pub fn hash(&self) -> [u8; 32]
    {
        Sha256::new()
            .chain(&self.serialize())
            .finalize()
            .as_slice()
            .try_into()
            .expect("Invalid length")
    }

    pub fn serialize(&self) -> Vec<u8>
    {
        let mut block: Vec<u8> = Vec::new();

        block.extend_from_slice(&self.index.to_be_bytes());
        block.extend_from_slice(&self.previous_hash);
        block.extend_from_slice(&self.timestamp.to_be_bytes());
        block.extend_from_slice(&self.nonce.to_be_bytes());

        // Serialize transactions

        block
    }
}
