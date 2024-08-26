#![allow(dead_code, unused_variables, unused_imports)]

use std::collections::HashMap;
use std::sync::Arc;
use std::usize;

use parking_lot::{Mutex, RwLock};
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;

use crate::block::{Block, BlockId, BlockManager};
use crate::thread::ThreadMgr;

// ====
pub type Tag = u8; // Tag  ::= 8-bit (fits a u8)

// ====
pub const NOP: Tag = 0x00; // NOTHING
pub const BND: Tag = 0x01; // BIND
pub const UND: Tag = 0x02; // UNBIND
pub const WIE: Tag = 0x03; // WRITE
pub const RAD: Tag = 0x04; // READ
pub const TAE: Tag = 0x05; // TAKE
pub const JON: Tag = 0x06; // JOIN
pub const SPL: Tag = 0x07; // SPLIT

// ====
pub const ADD: Tag = 0x10; // ADD
pub const SUB: Tag = 0x11; // SUB
pub const MUL: Tag = 0x12; // MUL
pub const DIV: Tag = 0x13; // DIV
pub const REM: Tag = 0x14; // REM
pub const AND: Tag = 0x15; // AND
pub const ORR: Tag = 0x16; // ORR
pub const XOR: Tag = 0x17; // XOR
pub const NOT: Tag = 0x18; // NOT
pub const EQ: Tag = 0x19; // EQUAL
pub const NEQ: Tag = 0x1A; // NOT EQUAL
pub const LT: Tag = 0x1B; // LESS THAN
pub const GT: Tag = 0x1C; // GREATER THAN
pub const LE: Tag = 0x1D; // LESS OR EQUAL
pub const GE: Tag = 0x1E; // GREATER OR EQUAL

// ====
pub const GO: Tag = 0x20; // GO
pub const GNZ: Tag = 0x21; // GO IF NOT ZERO
pub const GOZ: Tag = 0x22; // GO IF ZERO
pub const CLL: Tag = 0x23; // CALL
pub const RET: Tag = 0x24; // RETURN
pub const PRB: Tag = 0x30; // PARALLEL BEGIN
pub const PRE: Tag = 0x31; // PARALLEL END
pub const HLT: Tag = 0xFF; // HALF

// ====
pub struct Lem {
  pub block_manager: Arc<BlockManager>,
  pub instrs: Vec<Tag>,
  pub pointer: usize,
}

impl Lem {
  pub fn new(instrs: Vec<Tag>) -> Self {
    let block_manager = Arc::new(BlockManager::new());
    Lem { block_manager, instrs, pointer: 0 }
  }

  pub fn run(&mut self) {
    while self.pointer < self.instrs.len() {
      let instr = self.instrs[self.pointer];
      self.pointer += 1;
      match instr {
        NOP => {}
        BND => self.run_bnd(),
        UND => self.run_und(),
        WIE => self.run_wie(),
        RAD => self.run_rad(),
        TAE => self.run_tae(),
        ADD | SUB | MUL | DIV => self.run_arithmetic(instr),
        HLT => break,
        _ => panic!("ERROR >> unknown instruction {}", instr),
      }
    }
  }
  // Strings
  // ===
  pub fn write_string(&mut self, block_id: BlockId, index: usize, string: &str) {
    let mut values = Vec::new();
    for c in string.chars() {
      values.push(c as u32);
    }
    self.block_manager.update_block(block_id, values);
  }

  pub fn read_string(&self, block_id: BlockId) -> Option<String> {
    if let Some(block) = self.block_manager.get_block(block_id) {
      let string: String = block.slots.iter().filter_map(|&v| char::from_u32(v)).collect();
      return Some(string);
    }
    None
  }

  // Arrays
  // ======
  pub fn write_array(&mut self, block_id: BlockId, arrays: Vec<Vec<u32>>) {
    let mut values = Vec::new();
    for array in arrays {
      let sub_block_id = self.block_manager.create_block(array);
      values.push(sub_block_id as u32);
    }
    self.block_manager.update_block(block_id, values);
  }

  pub fn read_array(&self, block_id: BlockId) -> Vec<Vec<u32>> {
    let mut arrays = Vec::new();
    if let Some(block) = self.block_manager.get_block(block_id) {
      for &sub_block_id in &block.slots {
        if let Some(sub_block) = self.block_manager.get_block(sub_block_id as BlockId) {
          arrays.push(sub_block.slots.clone());
        }
      }
    }
    arrays
  }

  fn run_bnd(&mut self) {
    // allocate a new block of memory (todo: move to pool allocate memory)
    let values = vec![0; 10]; // allocating a block with 10 values free
    let block_id = self.block_manager.create_block(values);
    println!("LEM >> block id {} created", block_id);
  }

  fn run_und(&mut self) {
    // Libera um bloco de memória
    let block_id = self.fetch_block_id();
    self.block_manager.delete_block(block_id);
    println!("LEM >> block id {} deleted", block_id);
  }

  fn run_wie(&mut self) {
    // Escreve um valor em um bloco
    let block_id = self.fetch_block_id();
    let index = self.fetch_block_index();
    let value = self.fetch_value();
    if let Some(mut block) = self.block_manager.get_block(block_id) {
      block.set(index, value);
      println!("LEM >> block id {} index {} set to {}", block_id, index, value);
      self.block_manager.update_block(block_id, block.slots);
    }
  }

  fn run_rad(&mut self) {
    // Lê um valor de um bloco
    let block_id = self.fetch_block_id();
    let index = self.fetch_block_index();
    if let Some(block) = self.block_manager.get_block(block_id) {
      if let Some(value) = block.get(index) {
        println!("LEM >> block id {} index {} has value {}", block_id, index, value);
      } else {
        println!("LEM >> block id {} index {} out of bounds", block_id, index);
      }
    }
  }

  fn run_tae(&mut self) {
    // r um bloco (pode ser a mesma coisa que `UND` ou ter uma semântica diferente)
    // remove block (todo: move to pool free memory)
    let block_id = self.fetch_block_id();
    self.block_manager.delete_block(block_id);
    println!("LEM >> block id {} taken", block_id);
  }

  fn run_arithmetic(&mut self, instr: Tag) {
    let block_id = self.fetch_block_id();
    let index = self.fetch_block_index();
    let value = self.fetch_value();
    if let Some(mut block) = self.block_manager.get_block(block_id) {
      match instr {
        ADD => block.add(index, value),
        SUB => block.sub(index, value),
        MUL => block.mul(index, value),
        DIV => block.div(index, value),
        _ => {}
      }
      println!("LEM >> block id {} index {} updated", block_id, index);
      self.block_manager.update_block(block_id, block.slots);
    }
  }

  fn run_comparison(&mut self, instr: Tag, block: &mut Block, index: usize, value: u32) {
    let result = match instr {
      EQ => block.get(index) == Some(value),
      NEQ => block.get(index) != Some(value),
      LT => block.get(index).map_or(false, |v| v < value),
      GT => block.get(index).map_or(false, |v| v > value),
      LE => block.get(index).map_or(false, |v| v <= value),
      GE => block.get(index).map_or(false, |v| v >= value),
      _ => false,
    };
    // if comparison is true, store the result as 1 (true) or 0 (false)
    block.set(index, result as u32);
  }

  fn run_and(&mut self, block: &mut Block, index: usize, value: u32) {
    if let Some(existing) = block.get(index) {
      block.set(index, existing & value);
    }
  }

  fn run_or(&mut self, block: &mut Block, index: usize, value: u32) {
    if let Some(existing) = block.get(index) {
      block.set(index, existing | value);
    }
  }

  fn run_xor(&mut self, block: &mut Block, index: usize, value: u32) {
    if let Some(existing) = block.get(index) {
      block.set(index, existing ^ value);
    }
  }

  fn run_not(&mut self, block: &mut Block, index: usize) {
    if let Some(existing) = block.get(index) {
      block.set(index, !existing);
    }
  }

  fn fetch_block_id(&mut self) -> BlockId {
    let id = self.instrs[self.pointer] as BlockId;
    self.pointer += 1;
    id
  }

  fn fetch_block_index(&mut self) -> usize {
    let index = self.instrs[self.pointer] as usize;
    self.pointer += 1;
    index
  }

  fn fetch_value(&mut self) -> u32 {
    let value = self.instrs[self.pointer] as u32;
    self.pointer += 1;
    value
  }
}
