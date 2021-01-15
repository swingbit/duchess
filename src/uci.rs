use std::io::{self, BufRead};

use vampirc_uci::{UciMessage,UciSquare,UciMove,parse_one};
use crate::board::{Board,Pos,Move,Color};
use crate::minmax::{minmax};

impl Pos {
	pub fn from_uci(us: UciSquare) -> Option<Pos> {
		Self::from_coord(&us.to_string())
	}

	pub fn to_uci(&self) -> UciSquare {
		UciSquare {
			file: ((self.col as u8) + b'a') as char,
			rank: (self.row as u8) + 1,
		}
	}
}

impl Move {
	pub fn from_uci(um: UciMove) -> Move {
		if let Some(f_pos) = Pos::from_uci(um.from) {
			if let Some(t_pos) = Pos::from_uci(um.to) {
				return Move {f_pos, t_pos};
			}
		}
		panic!("Could not conver uci move: {}", um);
	}

	pub fn to_uci(&self) -> UciMove {
		UciMove {
			from: self.f_pos.to_uci(),
			to: self.t_pos.to_uci(),
			promotion: None
		}
	}
}


pub fn ucitest() {
	let mut b: Board = Board::new(Color::White);

	for line in io::stdin().lock().lines() {
		let msg: UciMessage = parse_one(&line.unwrap());
		match msg {
			UciMessage::Uci => {
					// Initialize the UCI mode of the chess engine.
					println!("{}", UciMessage::id_name("RustChess"));
					println!("{}", UciMessage::UciOk);
			},
			UciMessage::IsReady => {
				println!("{}", UciMessage::ReadyOk);
			},
			UciMessage::Position { startpos, fen, moves } => {
				b = Board::new(Color::White);
				for mv in moves.iter() {
					let lastmove = Move::from_uci(*mv);
					b = b.clone_apply_move(lastmove.f_pos, lastmove.t_pos);
				}
			},
			UciMessage::SetOption { name, value } => {
				// TODO
			},
			UciMessage::UciNewGame => {
				b = Board::new(Color::White);
			},
			UciMessage::Go { time_control, search_control } => {
				// TODO
				// play black
				let (_score, mv) = minmax(&b);
				let bestmove = UciMessage::BestMove { 
					best_move: mv.to_uci(),
					ponder: None 
				};
				b = b.clone_apply_move(mv.f_pos, mv.t_pos);
				println!("{}", bestmove);
			},
			_ => eprintln!(" Don't know what to do")
		}
	}
}
