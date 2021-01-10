use std::cmp;

use crate::board::{Board,Pos,Valuable};

const MAX_LEVELS:u8 = 5;
const USE_ALPHA_BETA_PRUNING:bool = true;

pub fn maximize(b: &Board, mut alpha:i32, beta:i32, level:u8) -> (i32, Option<(Pos,Pos)>) {
/* TODO:
  - if checkmate return i32::MAX
  - move ordering
*/
  if level == MAX_LEVELS {
    return (b.value(), None);
  }
  let mut best_score:i32 = i32::MIN;
  let mut best_move = None;
  let moves = b.generate_all_legal_moves();
  for (f_pos,t_pos) in moves {
    let child = b.clone_apply_move(&f_pos, &t_pos);
    let score = minimize(&child, alpha, beta, level+1).0;
    best_score = cmp::max(best_score, score);
    if level == 0 {
      best_move = Some((f_pos, t_pos));
    }
    if USE_ALPHA_BETA_PRUNING {
      alpha = cmp::max(alpha, best_score);
      if beta <= alpha {
        break;
      }
    }
  }
  (best_score, best_move)
}

pub fn minimize(b: &Board, alpha:i32, mut beta:i32, level:u8) -> (i32, Option<(Pos,Pos)>) {
/* TODO:
  - if checkmate return i32::MIN
  - move ordering
*/
  if level == MAX_LEVELS {
    return (b.value(), None);
  }
  let mut best_score:i32 = i32::MAX;
  let mut best_move = None;
  let moves = b.generate_all_legal_moves();
  for (f_pos,t_pos) in moves {
    let child = b.clone_apply_move(&f_pos, &t_pos);
    let score = maximize(&child, alpha, beta, level+1).0;
    best_score = cmp::min(best_score, score);
    if level == 0 {
      best_move = Some((f_pos, t_pos));
    }
    if USE_ALPHA_BETA_PRUNING {
      beta = cmp::min(beta,best_score);
      if beta <= alpha {
        break;
      }
    }
  }
  (best_score, best_move)
}
  