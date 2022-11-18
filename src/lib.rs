extern crate wasm_bindgen;
use std::str::FromStr;

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
use crate::board::{Board,Pos, MoveType};
use crate::negascout::negascout;
use crate::misc::OPTS_DEFAULT;

#[wasm_bindgen]
pub fn best_move(fromFEN:&str) -> String {
  let b = Board::from_fen(fromFEN).unwrap();
  let (_, mv) = negascout(&b, &OPTS_DEFAULT);
  return b.clone_apply_move(&mv).to_fen();
}

#[wasm_bindgen]
pub fn check_move(fromFEN:&str, fromPos:&str, toPos:&str) -> String {
  let b = match Board::from_fen(fromFEN) {
    Err(_) => return "illegal".to_string(),
    Ok(v) => v
  };
  let f_pos = match Pos::from_str(fromPos) {
    Err(_) => return "illegal".to_string(),
    Ok(v) => v
  };
  let t_pos = match Pos::from_str(toPos) {
    Err(_) => return "illegal".to_string(),
    Ok(v) => v
  };
  let mt = b.check_move(f_pos, t_pos, 0);
  match mt {
    MoveType::Illegal => return "illegal".to_string(),
    MoveType::Move => return "move".to_string(),
    MoveType::Capture(_) => return "capture".to_string(),
  }
}
