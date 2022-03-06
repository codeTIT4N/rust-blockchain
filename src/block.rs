use super::*;
use std::fmt::{self, Debug, Formatter};

// #[derive(Debug)]
pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String, //using string for now do we don't have to deal with references
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block[{}]: {} at: {} with: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload,
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,
        payload: String,
    ) -> Self {
        Block {
            index,
            timestamp,
            hash: vec![0; 32], //sha256 => 256 bits => 32 bytes
            prev_block_hash,
            nonce,
            payload,
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        //getting all the bytes in the block that need to be hashed
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(self.payload.as_bytes());
        bytes
    }
}
