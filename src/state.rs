use std::{sync::Arc, vec};
use tokio::sync::RwLock;
use crate::models::Block;

#[derive(Clone)]
pub struct AppState {
    pub blocks: Arc<RwLock<Vec<Block>>>,
    pub last_block: Arc<RwLock<Block>>,
}

impl AppState {
    pub fn new() -> Self {
        let new_block = Block::new();

        Self {
            blocks: Arc::new(RwLock::new(vec![new_block.clone()])),
            last_block: Arc::new(RwLock::new(new_block)),
        }
    }
}