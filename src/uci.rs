use std::io::{self, BufRead};
use vampirc_uci::{UciMessage,UciSquare,UciPiece,UciMove,UciInfoAttribute,parse_one};
use crate::board::{Board,Pos,Piece,Move,Color};
use crate::minimax::{minimax};
use crate::negamax::{negamax};
use crate::negascout::negascout;
use crate::misc::*;

impl Pos {
	pub fn from_uci(us: UciSquare) -> Pos {
		if let Some(pos) = Self::from_coord(&us.to_string()) {
			return pos;
		}
		panic!("Could not convert uci position: {}", us);
	}

	pub fn to_uci(&self) -> UciSquare {
		UciSquare {
			file: ((self.col as u8) + b'a') as char,
			rank: (self.row as u8) + 1,
		}
	}
}

impl Piece {
	pub fn from_uci(um: UciPiece) -> Piece {
		match um {
			UciPiece::Pawn => Piece::Pawn,
			UciPiece::Knight => Piece::Knight,
			UciPiece::Bishop => Piece::Bishop,
			UciPiece::Rook => Piece::Rook,
			UciPiece::Queen => Piece::Queen,
			UciPiece::King => Piece::King,
		}
	}
}

impl Move {
	pub fn from_uci(um: UciMove) -> Move {
		Move {
			f_pos: Pos::from_uci(um.from), 
			t_pos: Pos::from_uci(um.to),
		}
	}

	pub fn to_uci(&self) -> UciMove {
		UciMove {
			from: self.f_pos.to_uci(),
			to: self.t_pos.to_uci(),
			promotion: None
		}
	}
}
impl SearchInfo {
	pub fn to_uci(&self) -> UciMessage {
		let mut attributes: Vec<UciInfoAttribute> = Vec::new();
		attributes.push(UciInfoAttribute::from_centipawns(self.score_cp as i32));
		attributes.push(UciInfoAttribute::Depth(self.depth));
		if let Some(mv) = self.curr_move {
			attributes.push(UciInfoAttribute::CurrMove(mv.to_uci()));
		}
		attributes.push(UciInfoAttribute::Nodes(self.nodes));
		// attributes.push(UciInfoAttribute::Nps(self.nps));
		UciMessage::Info(attributes)
	}
}

pub fn uci_manager(opts: &Options) {
	let reader = io::BufReader::new(io::stdin());
	let mut lines = reader.lines();
	let mut b: Board = Board::new(Color::White);
	while let Some(Ok(line)) = lines.next() {
		// in_mgr_tx.send(line).await.unwrap();
		let msg: UciMessage = parse_one(&line);
		match msg {
			UciMessage::Uci => {
				// Initialize the UCI mode of the chess engine.
				println!("{}", UciMessage::Id {
					name: Some(String::from("RustChess")),
					author: Some(String::from("Roberto Cornacchia")),
				});
				println!("{}", UciMessage::UciOk);
			},
			UciMessage::IsReady => {
				println!("{}", UciMessage::ReadyOk);
			},
			UciMessage::Position { startpos:_, fen:_, moves } => {
				b = Board::new(Color::White);
				for mv in moves.iter() {
					let lastmove = Move::from_uci(*mv);
					b = b.clone_apply_move(&lastmove);
				}
			},
			UciMessage::SetOption { name:_, value:_ } => {
				// TODO
			},
			UciMessage::UciNewGame => {
				b = Board::new(Color::White);
			},
			UciMessage::Go { time_control:_, search_control:_} => {
				let opts = opts.clone();
				let b1 = b.clone();
				if let Some(res) = match opts.search_algo {
					SearchAlgorithm::Minimax => minimax(&b1, &opts),
					SearchAlgorithm::Negamax => negamax(&b1, &opts),
					SearchAlgorithm::Negascout => negascout(&b1, &opts)
				} {
					let _score = res.0;
					let mv = res.1;
					let bestmove = UciMessage::best_move(mv.to_uci());
					b = b.clone_apply_move(&mv);
					println!("{}", bestmove);
				} else {
					// TODO: communicate end of game. How?
				}
			},
			_ => eprintln!("Don't know what to do")
		}
	};
}
