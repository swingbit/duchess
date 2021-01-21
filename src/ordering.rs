use rand::thread_rng;
use rand::seq::SliceRandom;

use crate::board::{Board,Move};
use crate::evaluation::{Valuable};
use crate::misc::*;

pub fn move_ordering(bs: &mut Vec<(Move,Board)>, opts: &Options) {
	match opts.move_ordering {
		MoveOrdering::Eval => {
			bs.sort_by(|a,b| b.1.value().cmp(&a.1.value()));
		},
		MoveOrdering::Rand => {
			bs.shuffle(&mut thread_rng());
		},
		MoveOrdering::None => ()
	}
}