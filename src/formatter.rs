use std::fmt;

use crate::ast::Tree;

impl fmt::Display for Tree {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Tree::Var { name } => write!(f, "%{}", name),
      Tree::Lit { value } => write!(f, "{:?}", value),
      Tree::Add { lhs, rhs } => write!(f, "%tmp = add {}, {}", lhs, rhs),
      Tree::Sub { lhs, rhs } => write!(f, "%tmp = sub {}, {}", lhs, rhs),
      Tree::Mul { lhs, rhs } => write!(f, "%tmp = mul {}, {}", lhs, rhs),
      Tree::Div { lhs, rhs } => write!(f, "%tmp = div {}, {}", lhs, rhs),

      // Comparisons
      Tree::Eq { lhs, rhs } => write!(f, "%tmp = eq {}, {}", lhs, rhs),
      Tree::Neq { lhs, rhs } => write!(f, "%tmp = neq {}, {}", lhs, rhs),
      Tree::Gt { lhs, rhs } => write!(f, "%tmp = gt {}, {}", lhs, rhs),
      Tree::Lt { lhs, rhs } => write!(f, "%tmp = lt {}, {}", lhs, rhs),
      Tree::Ge { lhs, rhs } => write!(f, "%tmp = ge {}, {}", lhs, rhs),
      Tree::Le { lhs, rhs } => write!(f, "%tmp = le {}, {}", lhs, rhs),

      // Logical operations
      Tree::And { lhs, rhs } => write!(f, "%tmp = and {}, {}", lhs, rhs),
      Tree::Or { lhs, rhs } => write!(f, "%tmp = or {}, {}", lhs, rhs),
      Tree::Not { expr } => write!(f, "%tmp = not {}", expr),

      // Memory
      Tree::Bnd { dest, values } => {
        let values_str: Vec<String> = values.iter().map(|v| v.to_string()).collect();
        write!(f, "%{} = bnd [{}]", dest, values_str.join(", "))
      }
      Tree::Und { dest } => write!(f, "und %{}", dest),

      // Operations
      Tree::Wie { dest, value } => write!(f, "wie %{}, {}", dest, value),
      Tree::Rad { dest } => write!(f, "rad %{}", dest),
      Tree::Tae { dest } => write!(f, "tae %{}", dest),

      // Function calls and control flow
      Tree::Cll { name, args } => {
        let args_str: Vec<String> = args.iter().map(|arg| arg.to_string()).collect();
        write!(f, "cll %{}, [{}]", name, args_str.join(", "))
      }
      Tree::Run { value } => match value {
        Some(val) => write!(f, "run {}", val),
        None => write!(f, "run"),
      },
      Tree::Goo { cond, then, goz } => match goz {
        Some(goz_label) => write!(f, "goo {}, %{}, %{}", cond, then, goz_label),
        None => write!(f, "goo {}, %{}", cond, then),
      },
      Tree::Lop { cond, body } => write!(f, "lop {}, {}", cond, body),
    }
  }
}
