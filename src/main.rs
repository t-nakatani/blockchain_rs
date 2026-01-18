mod difficulty;

use serde::{Serialize, Deserialize};
use chrono::Utc;
use log::{error, info, warn, debug};
use difficulty::{hash2binary, DIFFICULTY_PREFIX};
use sha2::{Sha256, digest::Digest};
use std::thread;
use std::time::Duration;

pub struct App {
    pub blocks: Vec<Block>,
}

type Hash = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub id: u64,
    pub hash: Hash,
    pub previous_hash: Hash,
    pub timestamp: u64,
    pub data: String,
    pub nonce: u64,
}


impl App {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn genesis(&mut self) {
        let genesis_block = Block {
            id: 0,
            hash: Hash::from("0000000000000000000000000000000000000000000000000000000000000000"),
            previous_hash: Hash::from("genesis"),
            timestamp: Utc::now().timestamp() as u64,
            data: "This is the genesis block!".to_string(),
            nonce: 20260118,
        };
        self.blocks.push(genesis_block);
    }

    fn chain_is_valid(&self, chain: &[Block]) -> bool {
        for i in 1..chain.len() {
            if !self.is_valid_block(&chain[i - 1], &chain[i]) {
                return false;
            }
        }
        true
    }

    fn choose_longest_chain(&self, local_chain: Vec<Block>, remote_chain: Vec<Block>) -> Vec<Block> {
        let local_chain_is_valid = self.chain_is_valid(&local_chain);
        let remote_chain_is_valid = self.chain_is_valid(&remote_chain);

        if !local_chain_is_valid && !remote_chain_is_valid {
            panic!("Local and remote chains are both invalid");
        }

        if local_chain_is_valid && !remote_chain_is_valid {
            return local_chain;
        }

        if !local_chain_is_valid && remote_chain_is_valid {
            return remote_chain;
        }

        let longer_chain = if local_chain.len() >= remote_chain.len() {
            local_chain
        } else {
            remote_chain
        };

        longer_chain
    }

    fn try_add_block(&mut self, block: Block) {
        let last_block = self.blocks.last().unwrap();
        if self.is_valid_block(last_block, &block) {
            self.blocks.push(block);
        } else {
            error!("Invalid block");
        }
    }

    fn is_valid_block(&self, last_block: &Block, block: &Block) -> bool {
        if block.previous_hash != last_block.hash {
            warn!("Previous hash is not the same");
            return false;
        }

        if block.timestamp <= last_block.timestamp {
            warn!("Timestamp must be greater than the previous block");
            return false;
        }

        let hash_bytes = &hex::decode(&block.hash).unwrap();
        if !hash2binary(&hash_bytes).starts_with(DIFFICULTY_PREFIX) {
            warn!("Hash does not start with difficulty prefix, This mining is invalid");
            return false;
        }

        if hex::encode(calculate_hash(block.id, block.timestamp, &block.previous_hash, &block.data, block.nonce)) != block.hash {
            warn!("Block (id: {}) is invalid", block.id);
            return false;
        }

        true
    }
}


impl Block {
    fn new(id: u64, previous_hash: Hash, data: String) -> Self {
        let now = Utc::now().timestamp() as u64;
        let (nonce, hash) = mine_block(id, now, &previous_hash, &data);
        Self { id, previous_hash, timestamp: now, data, nonce, hash }
    }
}

fn mine_block(id: u64, timestamp: u64, previous_hash: &Hash, data: &String) -> (u64, Hash) {
    info!("Mining block (id: {})", id);
    let mut nonce = 0;

    loop {
        let hash = calculate_hash(id, timestamp, &previous_hash, &data, nonce);
        debug!("Hash: {}", hash2binary(&hash));
        if hash2binary(&hash).starts_with(DIFFICULTY_PREFIX) {
            info!("Block (id: {}) mined with nonce: {}", id, nonce);
            return (nonce, hex::encode(hash));
        }
        nonce += 1;
    }
}

fn calculate_hash(id: u64, timestamp: u64, previous_hash: &Hash, data: &String, nonce: u64) -> Vec<u8> {
    let data = serde_json::json!({ "id": id, "timestamp": timestamp, "previous_hash": previous_hash, "data": data, "nonce": nonce });
    let mut hasher = Sha256::new();
    hasher.update(data.to_string().as_bytes());
    hasher.finalize().as_slice().to_owned()
}


fn main() {
    env_logger::init();

    let mut app = App::new();
    app.genesis();

    let block_time = Duration::from_secs(1);
    for i in 1..10 {
        thread::sleep(block_time);
        let last_block = app.blocks.last().unwrap();
        let new_block = Block::new(i, last_block.hash.clone(), format!("This is the block {}", i));
        app.try_add_block(new_block);
    }
    // println!("{:?}", app.blocks);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_block() {
        let mut app = App::new();
        app.genesis();
        let new_block = Block {
            id: 1,
            hash: Hash::from("0000000000000000000000000000000000000000000000000000000000000001"),
            previous_hash: Hash::from("0000000000000000000000000000000000000000000000000000000000000000"),
            timestamp: Utc::now().timestamp() as u64 + 1,
            data: "This is the second block!".to_string(),
            nonce: 20260119,
        };
        assert!(app.is_valid_block(&app.blocks[0], &new_block));
    }
}