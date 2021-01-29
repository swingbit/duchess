// Pseudo-code
/*
function pvs(node, depth, α, β, color) is
		if depth = 0 or node is a terminal node then
				return color × the heuristic value of node
		for each child of node do
				if child is first child then
						score := −pvs(child, depth − 1, −β, −α, −color)
				else
						score := −pvs(child, depth − 1, −α − 1, −α, −color) (* search with a null window *)
						if α < score < β then
								score := −pvs(child, depth − 1, −β, −score, −color) (* if it failed high, do a full re-search *)
				α := max(α, score)
				if α ≥ β then
						break (* beta cut-off *)
		return α
*/

use std::cmp;

use crate::board::{Board, Color, Move};
use crate::evaluation::{Valuable, Value};
use crate::misc::*;
use crate::ordering::move_ordering;

pub fn negascout(
	b: &Board,
	tx: &Option<tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options,
) -> (Value, Move) {
	match b.player {
		Color::Black => {
			if let (v, Some(mv)) = negascout_search(b, Value::MIN + 1, Value::MAX - 1, 0, -1, tx, opts) {
				return (-v, mv);
			}
		}
		Color::White => {
			if let (v, Some(mv)) = negascout_search(b, Value::MIN + 1, Value::MAX - 1, 0, 1, tx, opts) {
				return (v, mv);
			}
		}
	}
	panic!("Couldn't find any move");
}

// https://homepage.iis.sinica.edu.tw/~tshsu/tcg/2018/slides/slide7.pdf
fn negascout_search(
	b: &Board,
	alpha: Value,
	beta: Value,
	depth: u8,
	sign: i8,
	tx: &Option<tokio::sync::mpsc::Sender<SearchInfo>>,
	opts: &Options,
) -> (Value, Option<Move>) {
	if depth == opts.max_depth {
		return (sign as Value * b.value(), None);
	}
	let moves = b.generate_all_legal_moves();
	let mut bs: Vec<(Move, Board)> = moves
		.iter()
		.map(|&x| (x, b.clone_apply_move(x.f_pos, x.t_pos)))
		.collect();

	move_ordering(&mut bs, sign, opts);

	let mut lower: Value = Value::MIN;
	let mut upper: Value = beta;
	let mut best_move = None;
	for (mv, child) in bs.iter() {
		let score = -negascout_search(child, -upper, -cmp::max(alpha,lower), depth + 1, -sign, tx, opts).0;
		if score > lower {
			if depth == 0 {
				best_move = Some(*mv);
			}
			if upper == beta || depth > (opts.max_depth - 3) || score >= beta {
				lower = score;
			} else {
				lower = -negascout_search(child, -beta, -score, depth + 1, -sign, tx, opts).0;
			}
		}
		if lower >= beta {
			break;
		}
		upper = cmp::max(alpha,lower) + 1;
	}
	(lower, best_move)
}