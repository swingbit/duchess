use std::cmp;
use async_recursion::async_recursion;

use crate::board::{Board, Color, Move};
use crate::evaluation::{Valuable, Value};
use crate::misc::*;
use crate::ordering::move_ordering;

pub async fn minimax(
	b: &Board,
	tx: Option<&tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options,
) -> (Value, Move) {
	let res;
	match b.player {
		Color::Black => res = minimize(b, Value::MIN, Value::MAX, 0, tx, opts).await,
		Color::White => res = maximize(b, Value::MIN, Value::MAX, 0, tx, opts).await,
	}
	if let (v, Some(mv)) = res {
		return (v, mv);
	}
	panic!("Couldn't find any move");
}

#[async_recursion]
async fn maximize(
	b: &Board,
	mut alpha: Value,
	beta: Value,
	depth: u8,
	tx: Option<&'async_recursion tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options,
) -> (Value, Option<Move>) {
	if depth == opts.max_depth {
		return (b.value(), None);
	}

	let mut bs: Vec<(Move, Board)> = b.generate_all();

	move_ordering(&mut bs, 1, opts);

	let mut best_score: Value = Value::MIN + 1;
	let mut best_move = None;
	for (mv, child) in bs.iter() {
		let score = minimize(child, alpha, beta, depth + 1, tx, opts).await.0;
		if score > best_score {
			if depth == 0 {
				best_move = Some(*mv);
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

#[async_recursion]
async fn minimize(
	b: &Board,
	alpha: Value,
	mut beta: Value,
	depth: u8,
	tx: Option<&'async_recursion tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options,
) -> (Value, Option<Move>) {
	if depth == opts.max_depth {
		return (b.value(), None);
	}

	let mut bs: Vec<(Move, Board)> = b.generate_all();

	move_ordering(&mut bs, 1, opts);

	let mut best_score: Value = Value::MAX - 1;
	let mut best_move = None;
	for (mv, child) in bs.iter() {
		let score = maximize(child, alpha, beta, depth + 1, tx, opts).await.0;
		if score < best_score {
			if depth == 0 {
				best_move = Some(*mv);
			}
			best_score = score;
		}
		if opts.alpha_beta {
			beta = cmp::min(beta, best_score);
			if beta <= alpha {
				break;
			}
		}
	}
	(best_score, best_move)
}
