use std::cmp;

use crate::board::{Board, Color, Move};
use crate::evaluation::{Valuable, Value};
use crate::misc::*;
use crate::ordering::move_ordering;

pub fn negamax(
	b: & Board,
	opts: &Options,
) -> (Value, Move) {
	let mut si = SearchInfo::new();

	match b.player {
		Color::Black => {
			if let (v, Some(mv)) = negamax_search(b, Value::MIN + 1, Value::MAX - 1, 0, -1, &mut si, opts) {
				return (-v, mv);
			}
		},
		Color::White => {
			if let (v, Some(mv)) = negamax_search(b, Value::MIN + 1, Value::MAX - 1, 0, 1, &mut si, opts) {
				return (v, mv);
			}
		}
	}
	panic!("Couldn't find any move");
}

fn negamax_search(
	b: &Board,
	mut alpha: Value,
	beta: Value,
	depth: u8,
	sign: i8,
	si: &mut SearchInfo,
	opts: &Options,
) -> (Value, Option<Move>) {
	if depth == opts.max_depth {
		return (sign as Value * b.value(), None);
	}
	si.nodes = si.nodes + 1;
	
	let mut bs: Vec<(Move, Board)> = b.generate_all();
	
	move_ordering(&mut bs, sign, opts);

	let mut best_score: Value = Value::MIN + 1;
	let mut best_move = None;
	for (mv, child) in bs.iter() {
		let score = -negamax_search(child, -beta, -alpha, depth + 1, -sign, si, opts).0;
		if score > best_score {
			best_score = score;
			if depth == 0 {
				best_move = Some(*mv);
			}
			if opts.alpha_beta {
				alpha = cmp::max(alpha, best_score);
				if alpha >= beta {
					break;
				}
			}
		}
	}
	(best_score, best_move)
}
