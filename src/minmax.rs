use std::cmp;

use crate::board::{Board, Pos, Move,Color};
use crate::evaluation::{Valuable,Value};

const MAX_LEVELS: u8 = 5;
const USE_ALPHA_BETA_PRUNING: bool = true;

pub fn minmax(b: &Board) -> (Value, Move) {
	let vp;
	match b.player {
		Color::Black => vp = minimize(&b, Value::MIN, Value::MAX, 0),
		Color::White => vp = maximize(&b, Value::MIN, Value::MAX, 0),
	}
	if let (v,Some((f_pos,t_pos))) = vp {
		return (v, Move {f_pos,t_pos});
	} else {
		panic!("Couldn't find any move");
	}

}

fn maximize(b: &Board, mut alpha: Value, beta: Value, level: u8) -> (Value, Option<(Pos, Pos)>) {
	/* TODO:
	   - if checkmate return Value::MAX
	   - move ordering
	*/
	if level == MAX_LEVELS {
		return (b.value(), None);
	}
	let mut best_score: Value = Value::MIN;
	let mut best_move = None;
	let moves = b.generate_all_legal_moves();
	for (f_pos, t_pos) in moves {
		let child = b.clone_apply_move(f_pos, t_pos);
		let score = minimize(&child, alpha, beta, level + 1).0;
		if score > best_score {
			if level == 0 {
				best_move = Some((f_pos, t_pos));
			}
			best_score = score;
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

fn minimize(b: &Board, alpha: Value, mut beta: Value, level: u8) -> (Value, Option<(Pos, Pos)>) {
	/* TODO:
	   - if checkmate return Value::MIN
	   - move ordering
	*/
	if level == MAX_LEVELS {
		return (b.value(), None);
	}
	let mut best_score: Value = Value::MAX;
	let mut best_move = None;
	let moves = b.generate_all_legal_moves();
	for (f_pos, t_pos) in moves {
		let child = b.clone_apply_move(f_pos, t_pos);
		let score = maximize(&child, alpha, beta, level + 1).0;
		if score < best_score {
			if level == 0 {
				best_move = Some((f_pos, t_pos));
			}
			best_score = score;
		}
		if USE_ALPHA_BETA_PRUNING {
			beta = cmp::min(beta, best_score);
			if beta <= alpha {
				break;
			}
		}
	}
	(best_score, best_move)
}
