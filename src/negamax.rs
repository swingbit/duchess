use std::cmp;

use crate::board::{Board, Color, Move};
use crate::evaluation::{Valuable, Value};
use crate::misc::*;
use crate::ordering::move_ordering;

pub fn negamax(
	b: &Board,
	tx: &Option<tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options,
) -> (Value, Move) {
	let res;
	match b.player {
		Color::Black => res = negamax_search(&b, Value::MIN, Value::MAX, 0, -1, tx, opts),
		Color::White => res = negamax_search(&b, Value::MIN, Value::MAX, 0, 1, tx, opts),
	}
	if let (v, Some(mv)) = res {
		match b.player {
			Color::Black => return (-v, mv),
			Color::White => return (v, mv),
		}
	} else {
		panic!("Couldn't find any move");
	}
}

fn negamax_search(
	b: &Board,
	mut alpha: Value,
	beta: Value,
	depth: u8,
	sign: i16,
	tx: &Option<tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options,
) -> (Value, Option<Move>) {
	if depth == opts.max_depth {
		return (sign * b.value(), None);
	}
	
	let moves = b.generate_all_legal_moves();
	let mut bs: Vec<(Move, Board)> = moves
		.iter()
		.map(|&x| (x, b.clone_apply_move(x.f_pos, x.t_pos)))
		.collect();
	move_ordering(&mut bs, opts);

	let mut best_score: Value = Value::MIN;
	let mut best_move = None;
	for (mv, child) in bs {
		let score = -negamax_search(&child, -beta, -alpha, depth + 1, -sign, tx, opts).0;
		if score > best_score {
			if depth == 0 {
				best_move = Some(mv);
			}
			best_score = score;
		}
		if opts.alpha_beta {
			alpha = cmp::max(alpha, best_score);
			if alpha >= beta {
				break;
			}
		}
	}
	(best_score, best_move)
}
