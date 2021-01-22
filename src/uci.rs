
use vampirc_uci::{UciMessage,UciSquare,UciMove,parse_one};
use crate::board::{Board,Pos,Move,Color};
use crate::evaluation::{Value};
use crate::minmax::{minmax};
use crate::negamax::{negamax};
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

pub async fn uci_manager(opts: &Options) {
	use tokio::sync::{mpsc};
	use tokio::io;
	use tokio::io::AsyncBufReadExt;

	let (sch_mgr_tx, mut sch_mgr_rx) = mpsc::channel(32);
	let (in_mgr_tx, mut in_mgr_rx) = mpsc::channel(8);

	// async stdin thread
	tokio::spawn(async move {
		let reader = io::BufReader::new(io::stdin());
		let mut lines = reader.lines();

		while let Ok(Some(line)) = lines.next_line().await {
			in_mgr_tx.send(line).await.unwrap();
		}
	});

	let mut b: Board = Board::new(Color::White);
	// process messages from multiple channels
	loop {
		tokio::select! {
			Some(line) = in_mgr_rx.recv() => {
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
							b = b.clone_apply_move(lastmove.f_pos, lastmove.t_pos);
						}
					},
					UciMessage::SetOption { name:_, value:_ } => {
						// TODO
					},
					UciMessage::UciNewGame => {
						b = Board::new(Color::White);
					},
					UciMessage::Go { time_control:_, search_control:_} => {
						// TODO: spawn this
						let res:(Value,Move);
						match opts.search_algo {
							SearchAlgorithm::Minmax => res = minmax(&b, &Some(sch_mgr_tx.clone()), opts),
							SearchAlgorithm::Negamax => res = negamax(&b, &Some(sch_mgr_tx.clone()), opts),
							_ => panic!("Algorithm {:?} not supported", opts.search_algo)
						}
						let _score = res.0;
						let mv = res.1;
						let bestmove = UciMessage::BestMove { 
							best_move: mv.to_uci(),
							ponder: None,
						};
						b = b.clone_apply_move(mv.f_pos, mv.t_pos);
						println!("{}", bestmove);
					},
					_ => eprintln!(" Don't know what to do")
				}
			}
			Some(line) = sch_mgr_rx.recv() => {
				println!("{:?}", line);
			}
		}
	}
}
