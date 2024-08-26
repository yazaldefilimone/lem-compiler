#![allow(dead_code)]
pub type BndTag = u16; // =: 16 bits (2^16 is the max number of binds)*

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Lit {
  Int(u32),
  Float(f64),
  BndId(BndTag),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Tree {
  Var { name: BndTag },                   //  eg: x
  Lit { value: Lit },                     // eg: 42
  Add { lhs: Box<Tree>, rhs: Box<Tree> }, // eg: x + y
  Sub { lhs: Box<Tree>, rhs: Box<Tree> }, // eg: x - y
  Mul { lhs: Box<Tree>, rhs: Box<Tree> }, // eg: x * y
  Div { lhs: Box<Tree>, rhs: Box<Tree> }, // eg: x / y

  // Comparisons
  Eq { lhs: Box<Tree>, rhs: Box<Tree> },  // eg: x == y
  Neq { lhs: Box<Tree>, rhs: Box<Tree> }, // eg: x != y
  Gt { lhs: Box<Tree>, rhs: Box<Tree> },  // eg: x > y
  Lt { lhs: Box<Tree>, rhs: Box<Tree> },  // eg: x < y
  Ge { lhs: Box<Tree>, rhs: Box<Tree> },  // eg: x >= y
  Le { lhs: Box<Tree>, rhs: Box<Tree> },  // eg: x <= y

  // Logical operations
  And { lhs: Box<Tree>, rhs: Box<Tree> }, // eg: x && y
  Or { lhs: Box<Tree>, rhs: Box<Tree> },  // eg: x || y
  Not { expr: Box<Tree> },                // eg: !x

  // Memory
  Bnd { dest: BndTag, values: Vec<Tree> }, // bind (alloc)
  Und { dest: BndTag },                    // unbind (free)

  // operations
  Wie { dest: BndTag, value: Box<Tree> }, // eg: x = 42
  Rad { dest: BndTag },                   // eg: x
  Tae { dest: BndTag },                   // eg: x
  // fn
  Cll { name: BndTag, args: Vec<Tree> },                      // eg: f(x, y)
  Run { value: Option<Box<Tree>> },                           // eg: return x
  Goo { cond: Box<Tree>, then: BndTag, goz: Option<BndTag> }, // eg: if x > y goto y else goto z
  Lop { cond: Box<Tree>, body: Box<Tree> },                   // eg: while x < y { ... }
}
