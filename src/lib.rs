extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

mod board;
mod evaluation;
mod inspection;
mod generation;
mod misc;
// mod minimax;
// mod negamax;
mod negascout;
mod ordering;
// mod uci;
use crate::board::{Board,Color};
use crate::negascout::negascout;
use crate::misc::OPTS_DEFAULT;

#[wasm_bindgen]
pub fn best_move(fen:&str) -> String {
  let b = Board::from_fen(fen).unwrap();
  let (_, mv) = negascout(&b, &OPTS_DEFAULT);
  return b.clone_apply_move(&mv).to_fen();
}
