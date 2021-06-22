use std::str::FromStr;
use crate::board::{Move};
use crate::evaluation::{Value};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SearchAlgorithm {
	Minimax,
	Negamax,
	Negascout,
}
impl FromStr for SearchAlgorithm {
	type Err = ();

	fn from_str(input: &str) -> Result<SearchAlgorithm, Self::Err> {
		match input.to_lowercase().as_str() {
			"minimax" => Ok(SearchAlgorithm::Minimax),
			"negamax" => Ok(SearchAlgorithm::Negamax),
			"negascout" => Ok(SearchAlgorithm::Negascout),
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

#[derive(Default,Debug, Copy, Clone, PartialEq, Eq)]
pub struct SearchInfo {
	pub depth: u8,
	pub curr_move: Option<Move>,
	pub score_cp: Value,
	pub nodes: u64,
	pub nps: u64
}
impl SearchInfo {
	pub fn new() -> Self {
			Default::default()
	}
}

#[allow(dead_code,non_snake_case)]
pub fn lift_Option<A, B>(f: impl Fn(A)->B) -> impl Fn(Option<A>)->Option<B> {
	move |a| Some(f(a?))
}
#[allow(dead_code,non_snake_case)]
pub fn lift2Option<A, B, C>(f: impl Fn(A, B)->C) -> impl Fn(Option<A>, Option<B>)->Option<C> {
	move |a, b| Some(f(a?, b?))
}