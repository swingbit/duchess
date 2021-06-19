use std::cmp;
use async_recursion::async_recursion;

use crate::board::{Board, Color, Move};
use crate::evaluation::{Valuable, Value};
use crate::misc::*;
use crate::ordering::move_ordering;

pub async fn negamax(
	b: & Board,
	tx: Option<&tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options,
) -> (Value, Move) {
	match b.player {
		Color::Black => {
			if let (v, Some(mv)) = negamax_search(b, Value::MIN + 1, Value::MAX - 1, 0, -1, tx, opts).await {
				return (-v, mv);
			}
		},
		Color::White => {
			if let (v, Some(mv)) = negamax_search(b, Value::MIN + 1, Value::MAX - 1, 0, 1, tx, opts).await {
				return (v, mv);
			}
		}
	}
	panic!("Couldn't find any move");
}

#[async_recursion]
async fn negamax_search(
	b: &Board,
	mut alpha: Value,
	beta: Value,
	depth: u8,
	sign: i8,
	tx: Option<&'async_recursion tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options,
) -> (Value, Option<Move>) {
	if depth == opts.max_depth {
		return (sign as Value * b.value(), None);
	}
	
	let mut bs: Vec<(Move, Board)> = b.generate_all();
	
	move_ordering(&mut bs, sign, opts);

	let mut best_score: Value = Value::MIN + 1;
	let mut best_move = None;
	for (mv, child) in bs.iter() {
		let score = -negamax_search(child, -beta, -alpha, depth + 1, -sign, tx, opts).await.0;
		if score > best_score {
			if depth == 0 {
				best_move = Some(*mv);
			}
			best_score = score;
			tx.unwrap().send(SearchInfo{depth, best_move: *mv}).await.expect("Sending error");
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
