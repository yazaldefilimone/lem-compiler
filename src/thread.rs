#![allow(dead_code, unused_variables, unused_imports)]

use parking_lot::Mutex;
use rayon::prelude::*;
use std::{sync::Arc, thread};

use crate::block::BlockManager;

pub struct ThreadMgr {
  pub pool: Arc<Mutex<BlockManager>>,
  pub tasks: Arc<Mutex<Vec<Box<dyn FnOnce() + Send>>>>,
  pub handles: Vec<thread::JoinHandle<()>>,
}

impl ThreadMgr {
  pub fn new() -> Self {
    ThreadMgr {
      pool: Arc::new(Mutex::new(BlockManager::new())),
      tasks: Arc::new(Mutex::new(Vec::new())),
      handles: Vec::new(),
    }
  }
  pub fn run(&mut self) {
    while let Some(task) = {
      let mut tasks = self.tasks.lock();
      tasks.pop()
    } {
      let handle = thread::spawn(move || {
        task(); // run the task
      });
      self.handles.push(handle);
    }
    for handle in self.handles.drain(..) {
      handle.join().expect("ERROR >> thread panicked :(");
    }
  }

  pub fn add_task<F>(&mut self, task: F)
  where
    F: FnOnce(&Mutex<BlockManager>) + Send + Sync + 'static,
  {
    let pool = Arc::clone(&self.pool);
    let task = Box::new(move || {
      task(&pool);
    });
    let mut tasks = self.tasks.lock();
    tasks.push(task);
  }
}
