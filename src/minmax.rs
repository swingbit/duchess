use std::cmp;

use crate::board::{Board,Move,Color};
use crate::evaluation::{Valuable,Value};
use crate::ordering::move_ordering;
use crate::misc::*;

const MAX_DEPTH: u8 = 5;
const USE_ALPHA_BETA_PRUNING: bool = true;


pub fn minmax(b: &Board, 
	tx: &Option<tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options) -> (Value, Move) {
	let res;
	match b.player {
		Color::Black => res = minimize(&b, Value::MIN, Value::MAX, 0, tx, opts),
		Color::White => res = maximize(&b, Value::MIN, Value::MAX, 0, tx, opts),
	}
	if let (v,Some(mv)) = res {
		return (v, mv);
	} else {
		panic!("Couldn't find any move");
	}

}

fn maximize(b: &Board, mut alpha: Value, beta: Value, depth: u8,
	tx: &Option<tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options) -> (Value, Option<Move>) {

	if depth == MAX_DEPTH {
		return (b.value(), None);
	}
	let mut best_score: Value = Value::MIN;
	let mut best_move = None;
	let moves = b.generate_all_legal_moves();
	let mut bs:Vec<(Move,Board)> = 
		moves.iter().map(|&x| (x, b.clone_apply_move(x.f_pos,x.t_pos))).collect();
	move_ordering(&mut bs, opts);

	for (mv, child) in bs {
		let score = minimize(&child, alpha, beta, depth + 1, tx, opts).0;
		if score > best_score {
			if depth == 0 {
				best_move = Some(mv);
			}
			best_score = score;
		}
		if USE_ALPHA_BETA_PRUNING {
			alpha = cmp::max(alpha, best_score);
			if alpha >= beta {
				break;
			}
		}
	}
	(best_score, best_move)
}

fn minimize(b: &Board, alpha: Value, mut beta: Value, depth: u8,
	tx: &Option<tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options) -> (Value, Option<Move>) {

	if depth == MAX_DEPTH {
		return (b.value(), None);
	}
	let mut best_score: Value = Value::MAX;
	let mut best_move = None;
	let moves = b.generate_all_legal_moves();
	let mut bs:Vec<(Move,Board)> = 
		moves.iter().map(|&x| (x, b.clone_apply_move(x.f_pos,x.t_pos))).collect();
	move_ordering(&mut bs, opts);

	for (mv, child) in bs {
		let score = maximize(&child, alpha, beta, depth + 1, tx, opts).0;
		if score < best_score {
			if depth == 0 {
				best_move = Some(mv);
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
