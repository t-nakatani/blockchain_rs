use serde::{Serialize, Deserialize};
use chrono::Utc;
use log::error;

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

    fn try_add_block(&mut self, block: Block) {
        let last_block = self.blocks.last().unwrap();
        if self.is_valid_block(last_block, &block) {
            self.blocks.push(block);
        }

        error!("Invalid block");
    }



    fn is_valid_block(&self, last_block: &Block, block: &Block) -> bool {
        if block.previous_hash != last_block.hash {
            return false;
        }

        if block.timestamp <= last_block.timestamp {
            return false;
        }

        true
    }
}


fn main() {
    let mut app = App::new();
    app.genesis();
    app.try_add_block(Block {
        id: 1,
        hash: Hash::from("0000000000000000000000000000000000000000000000000000000000000001"),
        previous_hash: Hash::from("0000000000000000000000000000000000000000000000000000000000000000"),
        timestamp: Utc::now().timestamp() as u64,
        data: "This is the second block!".to_string(),
        nonce: 20260119,
    });
    println!("{:?}", app.blocks);
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