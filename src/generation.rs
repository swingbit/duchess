use std::ops;

use crate::board::{Board,Pos,Move,MoveType,Piece,Color};

impl Board {
	pub fn generate_all(&self) -> Vec<(Move,Board)> {
		let mut all_moves = Vec::new();
		for c in 0..8 {
			for r in 0..8 {
				let f_pos = Pos::at(c,r).unwrap();
				if let Some(f_tile) = self.at(f_pos) {
					if f_tile.color == self.player {
						let moves = self.generate(f_pos);
						for t_pos in moves {
							let promotion;
							if f_tile.piece == Piece::Pawn 
									&& ((f_tile.color == Color::Black && t_pos.row == 0)
									||  (f_tile.color == Color::White && t_pos.row == 7)) {
								/* simplification: assume we always promote to a Queen */
								promotion = Some(Piece::Queen);
							} else {
								promotion = None;
							}

							let mv = Move{f_pos,t_pos, promotion};
							let b = self.clone_apply_move(&mv);
							if !b.is_king_in_check(self.player) {
								all_moves.push((mv,b));
							}
						}
					}
				}
			}
		}
		all_moves
	}

	pub fn generate(&self, f_pos: Pos) -> Vec<Pos> {
		let mut moves = Vec::new();
		
		if let Some(f_tile) = self.at(f_pos) {
			match f_tile.piece {
				Piece::Pawn => self.generate_pawn(&mut moves,f_pos),
				Piece::Knight => self.generate_knight(&mut moves,f_pos),
				Piece::Bishop => self.generate_bishop(&mut moves,f_pos,7),
				Piece::Rook  => self.generate_rook(&mut moves,f_pos,7),
				Piece::Queen => self.generate_queen(&mut moves,f_pos,7),
				Piece::King => self.generate_king(&mut moves,f_pos),
			}
		}
		moves
	}
	/// Generate possible Pawn moves from a given point
	pub fn generate_pawn(&self, moves: &mut Vec<Pos>, f_pos: Pos) {
		let f_incr:fn(i8,i8)->i8;
		let start_row:i8;

		if let Some(f_tile) = self.at(f_pos) {
			if f_tile.color == Color::White {
				f_incr = ops::Add::add;
				start_row = 1;
			} else {
				f_incr = ops::Sub::sub;
				start_row = 6;
			}

			/* forward by 1 */
			if let Some(t_pos) = Pos::at(f_pos.col, f_incr(f_pos.row,1)) {
				if self.move_type(f_pos,t_pos) == MoveType::Move {
					moves.push(t_pos);
					if f_pos.row == start_row {
						/* forward by 2 */
						if let Some(t_pos) = Pos::at(f_pos.col, f_incr(f_pos.row,2)) {
							if self.move_type(f_pos,t_pos) == MoveType::Move {
								moves.push(t_pos);
							}
						}
					}
				}
			}
			/* Capture diagonally */
			for i in [-1, 1].iter() {
				if let Some(t_pos) = Pos::at(f_pos.col+i, f_incr(f_pos.row,1)) {
					if let MoveType::Capture(_) = self.move_type(f_pos,t_pos) {
						moves.push(t_pos);
					}
				}
			}
		}
	}

	/// Generate possible Knight moves from a given point
	pub fn generate_knight(&self, moves: &mut Vec<Pos>, f_pos: Pos) {
		let tps = [
			(f_pos.col-1,f_pos.row-2),(f_pos.col-1,f_pos.row+2),
			(f_pos.col+1,f_pos.row-2),(f_pos.col+1,f_pos.row+2),
			(f_pos.col-2,f_pos.row-1),(f_pos.col-2,f_pos.row+1),
			(f_pos.col+2,f_pos.row-1),(f_pos.col+2,f_pos.row+1),
		];
		for (c,r) in tps.iter() {
			if let Some(t_pos) = Pos::at(*c,*r) {
				if self.move_type(f_pos,t_pos) != MoveType::Illegal {
					moves.push(t_pos);
				}
			}
		}
	}

	/// Generate possible straight and diagonal moves from a given point in any directions
	pub fn generate_arm(&self, moves: &mut Vec<Pos>, f_pos: Pos, max_len: i8, f_c:fn(i8,i8)->i8, f_r:fn(i8,i8)->i8) {
		for i in 1..max_len+1 {
			if let Some(t_pos) = Pos::at(f_c(f_pos.col,i), f_r(f_pos.row,i)) {
				match self.move_type(f_pos,t_pos) {
					MoveType::Move => moves.push(t_pos),
					MoveType::Capture(_) => { moves.push(t_pos); break; },
					MoveType::Illegal => break
				}
			} else { break; }
		}
	}

	/// Generate possible Bishop moves from a given point
	pub fn generate_bishop(&self, moves: &mut Vec<Pos>, f_pos: Pos, max_len: i8) {
		let add = ops::Add::add;
		let sub = ops::Sub::sub;

		/* north-west */
		self.generate_arm(moves,f_pos,max_len,sub,add);
		/* north-east */
		self.generate_arm(moves,f_pos,max_len,add,add);
		/* south-east */
		self.generate_arm(moves,f_pos,max_len,add,sub);
		/* south-west */
		self.generate_arm(moves,f_pos,max_len,sub,sub);
	}

	/// Generate possible Rook moves from a given point
	pub fn generate_rook(&self, moves: &mut Vec<Pos>, f_pos: Pos, max_len: i8) {
		let add = ops::Add::add;
		let sub = ops::Sub::sub;
		let id = |x,_| x;

		/* north */
		self.generate_arm(moves,f_pos,max_len,id,add);
		/* east */
		self.generate_arm(moves,f_pos,max_len,add,id);
		/* south */
		self.generate_arm(moves,f_pos,max_len,sub,id);
		/* west */
		self.generate_arm(moves,f_pos,max_len,id,sub);
	}

	/// Generate possible Queen moves from a given point
	pub fn generate_queen(&self, moves: &mut Vec<Pos>, f_pos: Pos, max_len: i8) {
		self.generate_bishop(moves,f_pos,max_len);
		self.generate_rook(moves,f_pos,max_len);
	}

	/// Generate possible King moves from a given point
	pub fn generate_king(&self, moves: &mut Vec<Pos>, f_pos: Pos) {
		self.generate_bishop(moves,f_pos,1);
		self.generate_rook(moves,f_pos,1);

		// /* castling */
		if self.can_castle_left[self.player as usize] {
			if self.at(Pos::at(f_pos.col-1,f_pos.row).unwrap()).is_none() &&
				self.at(Pos::at(f_pos.col-2,f_pos.row).unwrap()).is_none() &&
				!self.is_king_in_check(self.player) {
					moves.push(Pos::at(f_pos.col-2,f_pos.row).unwrap());
			}
		}
		if self.can_castle_right[self.player as usize] {
			if self.at(Pos::at(f_pos.col+1,f_pos.row).unwrap()).is_none() &&
				self.at(Pos::at(f_pos.col+2,f_pos.row).unwrap()).is_none() &&
				!self.is_king_in_check(self.player)  {
					moves.push(Pos::at(f_pos.col+2,f_pos.row).unwrap());
			}
		}
	}
}
