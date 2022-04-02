use clap::{Command, Arg};

mod board;
mod evaluation;
mod inspection;
mod generation;
mod minimax;
mod misc;
mod negamax;
mod negascout;
mod ordering;
mod uci;

use crate::board::{Board,Color,Move};
use crate::minimax::minimax;
use crate::negamax::negamax;
use crate::negascout::negascout;
use crate::evaluation::{Value};
use crate::uci::{uci_manager};
use crate::misc::*;


extern crate vampirc_uci;

async fn self_play_test(opts: &Options) {
	let mut b: Board = Board::new(Color::White);
	/* Just for testing: AI playing against itself in a loop */

	for i in 0..100 {
		let res:(Value,Move);

		match opts.search_algo {
			SearchAlgorithm::Minimax => res = minimax(&b, None, opts).await,
			SearchAlgorithm::Negamax => res = negamax(&b, None, opts).await,
			SearchAlgorithm::Negascout => res = negascout(&b, None, opts).await,
			// _ => panic!("Algorithm {:?} not supported", opts.search_algo)
		}
		let score = res.0;
		let mv = res.1;
		println!("{}. {:?}: [{}{}]({})", i, b.player, mv.f_pos, mv.t_pos, score);
		b = b.clone_apply_move(&mv);
	}
}

#[tokio::main]
async fn main() {
	let mut opts = OPTS_DEFAULT.clone();

	let matches = Command::new("Duchess")
		.version("0.1.0")
		.author("Roberto Cornacchia <roberto.cornacchia@gmail.com>")
		.about("A simple chess engine")
		.arg(
			Arg::new("ui")
				.long("ui")
				.takes_value(true)
				.possible_values(&["uci", "ansiterm"])
				.default_value("uci")
				.help("The UI talking to this engine"),
		)
		.arg(
			Arg::new("algo")
				.short('a')
				.long("algorithm")
				.takes_value(true)
				.possible_values(&["minimax", "negamax", "negascout"])
				.default_value("negascout")
				.help("Search algorithm"),
		)
		.arg(
			Arg::new("ord")
				.short('o')
				.long("ordering")
				.takes_value(true)
				.possible_values(&["none", "random", "eval"])
				.default_value("eval")
				.help("Move ordering strategy"),
		)
		.arg(
			Arg::new("depth")
				.short('d')
				.long("max-depth")
				.takes_value(true)
				.default_value("5")
				.help("Maximum depth of the search algorithm"),
		)
		.arg(
			Arg::new("no-alphabeta")
				.long("no-alphabeta")
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
		opts.alpha_beta = !matches.is_present("no-alphabeta");
		// println!("Options:\n {:#?}",opts);
		match opts.ui {
			Ui::Uci => uci_manager(&opts.clone()).await,
			Ui::AnsiTerm => self_play_test(&opts.clone()).await,
		}

}
