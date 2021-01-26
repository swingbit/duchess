use clap::{App, Arg};

mod board;
mod evaluation;
mod generation;
mod minimax;
mod misc;
mod negamax;
mod ordering;
mod uci;

use crate::board::{Board,Color,Move};
use crate::minimax::minimax;
use crate::negamax::negamax;
use crate::evaluation::{Value};
use crate::uci::{uci_manager};
use crate::misc::*;


extern crate vampirc_uci;

fn self_play_test(opts: &Options) {
	let mut b: Board = Board::new(Color::White);
	/* Just for testing: AI playing against itself in a loop */

	for _ in 0..10 {
		let res:(Value,Move);

		match opts.search_algo {
			SearchAlgorithm::Minmax => res = minimax(&b, &None, opts),
			SearchAlgorithm::Negamax => res = negamax(&b, &None, opts),
			_ => panic!("Algorithm {:?} not supported", opts.search_algo)
		}
		let score = res.0;
		let mv = res.1;
		println!("{:?}: [{}{}]({})", b.player, mv.f_pos, mv.t_pos, score);
		b = b.clone_apply_move(mv.f_pos, mv.t_pos);
	}
}

#[tokio::main]
async fn main() {
	let mut opts = OPTS_DEFAULT.clone();

	let matches = App::new("RustChess")
		.version("0.1.0")
		.author("Roberto Cornacchia <roberto.cornacchia@gmail.com>")
		.about("A simple chess engine")
		.arg(
			Arg::with_name("ui")
				.long("ui")
				.takes_value(true)
				.possible_values(&["uci", "ansiterm"])
				.default_value("uci")
				.help("The UI talking to this engine"),
		)
		.arg(
			Arg::with_name("algo")
				.short("a")
				.long("algorithm")
				.takes_value(true)
				.possible_values(&["minmax", "negamax"])
				.default_value("negamax")
				.help("Search algorithm"),
		)
		.arg(
			Arg::with_name("ord")
				.short("o")
				.long("ordering")
				.takes_value(true)
				.possible_values(&["none", "random", "eval"])
				.default_value("eval")
				.help("Move ordering strategy"),
		)
		.arg(
			Arg::with_name("depth")
				.short("d")
				.long("max-depth")
				.takes_value(true)
				.default_value("5")
				.help("Maximum depth of the search algorithm"),
		)
		.arg(
			Arg::with_name("disable-alphabeta")
				.long("disable-alphabeta")
				.help("Disable alpha-beta pruning"),
		)
		.get_matches();

		if let Some(m) = matches.value_of("ui") {
			if let Ok(val) = m.parse::<Ui>() {
				opts.ui = val;
			}
		}
		if let Some(m) = matches.value_of("algo") {
			if let Ok(val) = m.parse::<SearchAlgorithm>() {
				opts.search_algo = val;
			}
		}
		if let Some(m) = matches.value_of("ord") {
			if let Ok(val) = m.parse::<MoveOrdering>() {
				opts.move_ordering = val;
			}
		}
		if let Some(m) = matches.value_of("depth") {
			if let Ok(val) = m.parse::<u8>() {
				opts.max_depth = val;
			}
		}
		opts.alpha_beta = !matches.is_present("disable-alphabeta");
		// println!("Options:\n {:#?}",opts);
		match opts.ui {
			Ui::Uci => uci_manager(&opts.clone()).await,
			Ui::AnsiTerm => self_play_test(&opts.clone()),
		}

}
