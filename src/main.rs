use lem::{Lem, ADD, BND, HLT, RAD, WIE};
mod ast;
mod block;
mod formatter;
mod lem;
mod pool;
mod thread;

// draft string to int array (maybe we use in the future)
pub fn convert_string_to_int_array(s: &str) -> Vec<u32> {
  s.chars().map(|c| c as u32).collect()
}

pub fn compare_arrays(a: &[u32], b: &[u32]) -> bool {
  a == b
}

pub fn convert_int_array_to_string(arr: &[u32]) -> String {
  arr.iter().map(|&c| std::char::from_u32(c).unwrap()).collect()
}

pub fn concatenate_arrays(a: &[u32], b: &[u32]) -> Vec<u32> {
  [a, b].concat()
}

fn main() {
  let instrs = vec![
    BND, // cria um bloco
    WIE, 0, 1, 42, // escreve 42 no índice 1 do bloco 0
    RAD, 0, 1, // le o valor do índice 1 do bloco 0
    ADD, 0, 1, 10, // adiciona 10 ao valor no índice 1 do bloco 0
    RAD, 0, 1,   // le o valor atualizado do índice 1 do bloco 0
    HLT, // ...
  ];
  let mut vm = Lem::new(instrs);
  vm.run();
}
