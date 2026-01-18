use serde::{Serialize, Deserialize};
use chrono::Utc;


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
}


fn main() {
    let mut app = App::new();
    app.genesis();
    println!("{:?}", app.blocks);
}
