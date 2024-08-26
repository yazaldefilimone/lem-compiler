use std::collections::HashMap;

use parking_lot::RwLock;

use crate::block::{Block, BlockId};

pub struct Pool {
  free_blocks: Vec<BlockId>,
  pool_size: usize,
  pub blocks: RwLock<HashMap<BlockId, Block>>,
}

impl Pool {
  pub fn new(pool_size: usize) -> Self {
    let free_blocks = Vec::with_capacity(pool_size);
    let blocks = RwLock::new(HashMap::with_capacity(pool_size));
    Pool { free_blocks, pool_size, blocks }
  }

  pub fn allocate(&mut self, values: Vec<u32>) -> BlockId {
    if let Some(block_id) = self.free_blocks.pop() {
      self.blocks.write().insert(block_id, Block::new(values));
      block_id
    } else {
      let block_id = self.blocks.read().len();
      self.blocks.write().insert(block_id, Block::new(values));
      block_id
    }
  }
  pub fn free(&mut self, block_id: BlockId) {
    self.blocks.write().remove(&block_id);
    self.free_blocks.push(block_id);
  }
}
