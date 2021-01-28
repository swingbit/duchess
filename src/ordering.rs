use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::board::{Board, Move};
use crate::evaluation::{Valuable,Value};
use crate::misc::*;

pub fn move_ordering(bs: &mut Vec<(Move, Board)>, sign: i8, opts: &Options) {
	match opts.move_ordering {
		MoveOrdering::Eval => {
			bs.sort_unstable_by(|a, b| (sign as Value * b.1.value()).cmp( &(sign as Value * a.1.value())) );
		},
		MoveOrdering::Rand => {
			bs.shuffle(&mut thread_rng());
		},
		MoveOrdering::None => (),
	}
}
