use parking_lot::{Mutex, RwLock};
use std::collections::HashMap;
use std::sync::Arc;

pub type BlockId = usize;

#[derive(Debug, Clone)]
pub struct Block {
  pub slots: Vec<u32>,
}

impl Block {
  pub fn new(values: Vec<u32>) -> Self {
    Block { slots: values }
  }

  pub fn get(&self, index: usize) -> Option<u32> {
    self.slots.get(index).copied()
  }

  pub fn set(&mut self, index: usize, value: u32) {
    if index < self.slots.len() {
      self.slots[index] = value;
    }
  }

  pub fn add(&mut self, index: usize, value: u32) {
    if let Some(elem) = self.slots.get_mut(index) {
      *elem += value;
    }
  }

  pub fn sub(&mut self, index: usize, value: u32) {
    if let Some(elem) = self.slots.get_mut(index) {
      *elem -= value;
    }
  }

  pub fn mul(&mut self, index: usize, value: u32) {
    if let Some(elem) = self.slots.get_mut(index) {
      *elem *= value;
    }
  }

  pub fn div(&mut self, index: usize, value: u32) {
    if let Some(elem) = self.slots.get_mut(index) {
      *elem /= value;
    }
  }
}

#[derive(Debug)]
pub struct BlockManager {
  pub blocks: RwLock<HashMap<BlockId, Block>>,
}

impl BlockManager {
  pub fn new() -> Self {
    BlockManager { blocks: RwLock::new(HashMap::new()) }
  }

  pub fn create_block(&self, values: Vec<u32>) -> BlockId {
    let mut blocks = self.blocks.write();
    let id = blocks.len();
    blocks.insert(id, Block::new(values));
    id
  }

  pub fn get_block(&self, id: BlockId) -> Option<Block> {
    let blocks = self.blocks.read();
    blocks.get(&id).cloned()
  }

  pub fn update_block(&self, id: BlockId, values: Vec<u32>) {
    let mut blocks = self.blocks.write();
    if let Some(block) = blocks.get_mut(&id) {
      block.slots = values;
    }
  }

  pub fn delete_block(&self, id: BlockId) {
    let mut blocks = self.blocks.write();
    blocks.remove(&id);
  }
}
