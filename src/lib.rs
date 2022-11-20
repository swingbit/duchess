extern crate wasm_bindgen;
use std::str::FromStr;

use clap::ColorChoice;
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
use crate::board::{Board,Pos, Move, MoveType, GameEnd, Color};
use crate::negascout::negascout;
use crate::misc::OPTS_DEFAULT;

#[wasm_bindgen]
/// Computes the best move from the given board
pub fn find_best_move(fromFEN:&str) -> String {
  let b = Board::from_fen(fromFEN).unwrap();
  let (_, mv) = negascout(&b, &OPTS_DEFAULT);
  return b.clone_apply_move(&mv).to_fen();
}

#[wasm_bindgen]
/// Checks that the move is legal e returns a new FEN for the opponent
pub fn make_move(fromFEN:&str, fromPos:&str, toPos:&str) -> String {
  let b = match Board::from_fen(fromFEN) {
    Err(_) => return "illegal_input".to_string(),
    Ok(v) => v
  };
  let f_pos = match Pos::from_str(fromPos) {
    Err(_) => return "illegal_input".to_string(),
    Ok(v) => v
  };
  let t_pos = match Pos::from_str(toPos) {
    Err(_) => return "illegal_input".to_string(),
    Ok(v) => v
  };
  let mt = b.check_move(f_pos, t_pos, 0);
  match mt {
    MoveType::Illegal => return "illegal".to_string(),
    _ => {
      return b.clone_apply_move(&Move{f_pos, t_pos}).to_fen();
    }
  }
}

#[wasm_bindgen]
/// Reports whether and in what way the game ended
pub fn check_end_game(fromFEN:&str) -> String {
  let b = match Board::from_fen(fromFEN) {
    Err(_) => return "illegal_input".to_string(),
    Ok(v) => v
  };
  match b.check_end_game() {
    Some(GameEnd::Draw) => return "draw".to_string(),
    Some(GameEnd::Checkmate(Color::Black)) => return "checkmate black".to_string(),
    Some(GameEnd::Checkmate(Color::White)) => return "checkmate white".to_string(),
    None => return "none".to_string()
  };
}