use std::cmp;

use crate::board::{Board, Color, Move};
use crate::evaluation::{Valuable, Value};
use crate::misc::*;
use crate::ordering::move_ordering;

pub fn negascout(
	b: &Board,
	opts: &Options,
) -> Option<(Value, Move)> {
	match b.player {
		Color::Black => {
			if let (v, Some(mv)) = negascout_search(b, Value::MIN + 1, Value::MAX - 1, 0, -1, opts) {
				return Some((-v, mv));
			}
		}
		Color::White => {
			if let (v, Some(mv)) = negascout_search(b, Value::MIN + 1, Value::MAX - 1, 0, 1, opts) {
				return Some((v, mv));
			}
		}
	}
	None
}

// Implementation inspired to https://homepage.iis.sinica.edu.tw/~tshsu/tcg/2018/slides/slide7.pdf
fn negascout_search(
	b: &Board,
	alpha: Value,
	beta: Value,
	depth: u8,
	sign: i8,
	opts: &Options,
) -> (Value, Option<Move>) {
	if depth == opts.max_depth {
		return (sign as Value * b.value(), None);
	}
	
	let mut bs: Vec<(Move, Board)> = b.generate_all();

	move_ordering(&mut bs, sign, opts);

	let mut lower: Value = Value::MIN + 1;
	let mut upper: Value = beta;
	let mut best_move = None;
	for (mv, child) in bs.iter() {
		let score = -negascout_search(child, -upper, -cmp::max(alpha,lower), depth + 1, -sign, opts).0;
		if score > lower {
			if depth == 0 {
				best_move = Some(*mv);
			}
			if upper == beta || depth > (opts.max_depth - 3) || score >= beta {
				lower = score;
			} else {
				lower = -negascout_search(child, -beta, -score, depth + 1, -sign, opts).0;
			}
		}
		if lower >= beta {
			break;
		}
		upper = cmp::max(alpha,lower) + 1;
	}
	(lower, best_move)
}
