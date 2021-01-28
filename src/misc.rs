use std::str::FromStr;
use crate::board::{Move};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SearchAlgorithm {
	Minimax,
	Negamax,
	Pvs,
}
impl FromStr for SearchAlgorithm {
	type Err = ();

	fn from_str(input: &str) -> Result<SearchAlgorithm, Self::Err> {
		match input.to_lowercase().as_str() {
			"minimax" => Ok(SearchAlgorithm::Minimax),
			"negamax" => Ok(SearchAlgorithm::Negamax),
			"pvs" => Ok(SearchAlgorithm::Pvs),
			_ => Err(()),
		}
	}
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveOrdering {
	None,
	Rand,
	Eval,
}
impl FromStr for MoveOrdering {
	type Err = ();

	fn from_str(input: &str) -> Result<MoveOrdering, Self::Err> {
		match input.to_lowercase().as_str() {
			"none" => Ok(MoveOrdering::None),
			"random" => Ok(MoveOrdering::Rand),
			"eval" => Ok(MoveOrdering::Eval),
			_ => Err(()),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ui {
	Uci,
	AnsiTerm,
}
impl FromStr for Ui {
	type Err = ();

	fn from_str(input: &str) -> Result<Ui, Self::Err> {
		match input.to_lowercase().as_str() {
			"uci" => Ok(Ui::Uci),
			"ansiterm" => Ok(Ui::AnsiTerm),
			_ => Err(()),
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Options {
	pub search_algo: SearchAlgorithm,
	pub move_ordering: MoveOrdering,
	pub max_depth: u8,
	pub ui: Ui,
	pub alpha_beta: bool,
}

pub static OPTS_DEFAULT: Options = Options {
	search_algo: SearchAlgorithm::Negamax,
	move_ordering: MoveOrdering::Eval,
	max_depth: 5,
	ui: Ui::Uci,
	alpha_beta: true,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SearchInfo {
	pub depth: u8,
	pub best_move: Move,
}
