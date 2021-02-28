use std::ops;

use crate::board::{Board,Pos,Move,MoveType,Piece,Color};

impl Board {
	fn move_type(&self, f_pos: Pos, t_pos: Pos) -> MoveType {
		if let Some(f_tile) = self.at(f_pos) {
			if let Some(t_tile) = self.at(t_pos) {
				if f_tile.color != t_tile.color {
					return MoveType::Capture;
				}
			} else {
				return MoveType::Move;
			}
		}
		MoveType::Illegal
	}

	/// Checks whether a move is valid.
	/// Not needed for generated moves, only for human moves.
	/// Allows to "see through" a specified number of obstacles
	#[allow(dead_code)]
	pub fn check_move(&self, f_pos: Pos, t_pos: Pos, max_obstacles: u8) -> MoveType {

		/* Checks possible moves from a given point in all directions */
		fn check_arm(b: &Board, f_pos: Pos, t_pos: Pos, max_obstacles: u8, 
					f_c:fn(i8,i8)->i8, f_r:fn(i8,i8)->i8) -> MoveType {
			let mut obstacles = 0;
			for i in 1..8 {
				if let Some(x_pos) = Pos::at(f_c(f_pos.col,i), f_r(f_pos.row,i)) {
					let move_type = b.move_type(f_pos,x_pos);
					match &move_type {
						MoveType::Illegal | MoveType::Capture => {
							if obstacles == max_obstacles {
								if t_pos == x_pos {
									return move_type;
								} else {
									return MoveType::Illegal;
								}
							};
							obstacles = obstacles + 1;
						},
						MoveType::Move => {
							if t_pos == x_pos {
								return move_type;
							}
						}
					}
				} else { 
					return MoveType::Illegal; 
				}
			}
			MoveType::Illegal
		};

		fn check_bishop(b: &Board, f_pos: Pos, t_pos: Pos, max_obstacles: u8) -> MoveType {
			let add = ops::Add::add;
			let sub = ops::Sub::sub;
			
			if (t_pos.col - f_pos.col).abs() != (t_pos.row - f_pos.row).abs () {
				return MoveType::Illegal;
			}
			if t_pos.col < f_pos.col && t_pos.row > f_pos.row {
				/* north-west */
				return check_arm(b,f_pos,t_pos,max_obstacles,sub,add);
			}
			if t_pos.col > f_pos.col && t_pos.row > f_pos.row {
				/* north-east */
				return check_arm(b,f_pos,t_pos,max_obstacles,add,add);
			}
			if t_pos.col > f_pos.col && t_pos.row < f_pos.row {
				/* south-east */
				return check_arm(b,f_pos,t_pos,max_obstacles,add,sub);
			}
			if t_pos.col < f_pos.col && t_pos.row < f_pos.row {
				/* south-west */
				return check_arm(b,f_pos,t_pos,max_obstacles,sub,sub);
			}
			MoveType::Illegal
		}

		fn check_rook(b: &Board, f_pos: Pos, t_pos: Pos, max_obstacles: u8) -> MoveType {
			let add = ops::Add::add;
			let sub = ops::Sub::sub;
			let id = |x,_| x;
			
			if t_pos.col == f_pos.col && t_pos.row > f_pos.row {
				/* north */
				return check_arm(b,f_pos,t_pos,max_obstacles,id,add);
			}
			if t_pos.col > f_pos.col && t_pos.row == f_pos.row {
				/* east */
				return check_arm(b,f_pos,t_pos,max_obstacles,add,id);
			}
			if t_pos.col == f_pos.col && t_pos.row < f_pos.row {
				/* south */
				return check_arm(b,f_pos,t_pos,max_obstacles,id,sub);
			}
			if t_pos.col < f_pos.col && t_pos.row == f_pos.row {
				/* west */
				return check_arm(b,f_pos,t_pos,max_obstacles,sub,id);
			}
			MoveType::Illegal
		}

		if let Some(f_tile) = self.at(f_pos) {
			match f_tile.piece {
				Piece::Pawn => {
					let f_incr:fn(i8,i8)->i8;
					let start_row:i8;

					if f_tile.color == Color::White {
						f_incr = ops::Add::add;
						start_row = 1;
					} else {
						f_incr = ops::Sub::sub;
						start_row = 6;
					}
					if let Some(t_tile) = self.at(t_pos) {
						/* Capture diagonally */
						if (t_pos.col -f_pos.col).abs() == 1 &&
								t_pos.row == f_incr(f_pos.row,1) &&
								t_tile.color != f_tile.color {
							return MoveType::Capture;
						}
					} else {
						/* forward by 1 or 2 */
						if t_pos.col == f_pos.col {
							if t_pos.row == f_incr(f_pos.row,1) ||
									(f_pos.row == start_row && 
										self.at(Pos::at(f_pos.col,f_incr(start_row,1)).unwrap()).is_none() 
										&& t_pos.row == f_incr(start_row,2)) {
								return MoveType::Move;
							}
						}
					}
				},

				Piece::Knight => {
					if ((f_pos.row-t_pos.row).abs() == 1 && (f_pos.col-t_pos.col).abs() == 2) ||
						 ((f_pos.row-t_pos.row).abs() == 2 && (f_pos.col-t_pos.col).abs() == 1) {
						if let Some(t_tile)	= self.at(t_pos) {
							if t_tile.color != f_tile.color {
								return MoveType::Capture;
							}
						} else {
							return MoveType::Move;
						}
					}
				},

				Piece::Bishop => {
					return check_bishop(self,f_pos,t_pos,max_obstacles);
				},

				Piece::Rook => {
					return check_rook(self,f_pos,t_pos,max_obstacles);
				},

				Piece::Queen => {
					let cb = check_bishop(self,f_pos,t_pos,max_obstacles);
					if cb != MoveType::Illegal {
						return cb;
					}
					return check_rook(self,f_pos,t_pos,max_obstacles);
				},

				Piece::King => {
					if (f_pos.col - t_pos.col).abs() == 2 {
						/* castling (TODO: cannot be used in currently in check) */
						match self.player {
							Color::Black => {
								if !self.can_castle_left[self.player as usize] && t_pos.col == 6 {
									return MoveType::Illegal;
								}
								if !self.can_castle_right[self.player as usize] && t_pos.col == 2 {
									return MoveType::Illegal;
								}
							},
							Color::White => {
								if !self.can_castle_left[self.player as usize] && t_pos.col == 2 {
									return MoveType::Illegal;
								}
								if !self.can_castle_right[self.player as usize] && t_pos.col == 6 {
									return MoveType::Illegal;
								}
							}
						}
						if self.at(Pos::at(t_pos.col,f_pos.row).unwrap()).is_none() &&
							 self.at(Pos::at((f_pos.col + t_pos.col)/2,f_pos.row).unwrap()).is_none() {
							return MoveType::Move;
						}
						return MoveType::Illegal;
					}

					let cb = check_bishop(self,f_pos,t_pos,max_obstacles);
					if cb != MoveType::Illegal {
						return cb;
					}
					return check_rook(self,f_pos,t_pos,max_obstacles);
				},
			}
		}
		MoveType::Illegal
	}

	
	pub fn generate_all_legal_moves(&self) -> Vec<Move> {
		let mut all_moves = Vec::new();
		for c in 0..8 {
			for r in 0..8 {
				let f_pos = Pos::at(c,r).unwrap();
				if let Some(f_tile) = self.at(f_pos) {
					if f_tile.color == self.player {
						let moves = self.generate_legal_moves(f_pos);
						for t_pos in moves {
							let promotion;
							if f_tile.piece == Piece::Pawn 
							 && ((f_tile.color == Color::Black && f_pos.row == 0)
							  || (f_tile.color == Color::White && f_pos.row == 7)) {
										/* simplification: assume we always promote to a Queen */
								promotion = Some(Piece::Queen);
							} else {
								promotion = None;
							}
							all_moves.push(Move{f_pos,t_pos, promotion});
						}
					}
				}
			}
		}
		all_moves
	}

	pub fn generate_legal_moves(&self, f_pos: Pos) -> Vec<Pos> {
		let mut moves = Vec::new();

		/// Generate possible moves from a given point in all directions
		fn generate_arm(moves: &mut Vec<Pos>, b: &Board, f_pos: Pos, max_len: i8, f_c:fn(i8,i8)->i8, f_r:fn(i8,i8)->i8) {
			for i in 1..max_len+1 {
				if let Some(t_pos) = Pos::at(f_c(f_pos.col,i), f_r(f_pos.row,i)) {
					match b.move_type(f_pos,t_pos) {
						MoveType::Move => moves.push(t_pos),
						MoveType::Capture => { moves.push(t_pos); break; },
						MoveType::Illegal => break
					}
				} else { break; }
			}
		};

		/// Generate possible bishop moves from a given point
		fn generate_bishop(moves: &mut Vec<Pos>, b: &Board, f_pos: Pos, max_len: i8) {
			let add = ops::Add::add;
			let sub = ops::Sub::sub;

			/* north-west */
			generate_arm(moves,b,f_pos,max_len,sub,add);
			/* north-east */
			generate_arm(moves,b,f_pos,max_len,add,add);
			/* south-east */
			generate_arm(moves,b,f_pos,max_len,add,sub);
			/* south-west */
			generate_arm(moves,b,f_pos,max_len,sub,sub);
		}

		/// Generate possible rook moves from a given point
		fn generate_rook(moves: &mut Vec<Pos>, b: &Board, f_pos: Pos, max_len: i8) {
			let add = ops::Add::add;
			let sub = ops::Sub::sub;
			let id = |x,_| x;

			/* north */
			generate_arm(moves,b,f_pos,max_len,id,add);
			/* east */
			generate_arm(moves,b,f_pos,max_len,add,id);
			/* south */
			generate_arm(moves,b,f_pos,max_len,sub,id);
			/* west */
			generate_arm(moves,b,f_pos,max_len,id,sub);
		}
		
		if let Some(f_tile) = self.at(f_pos) {
			match f_tile.piece {
				Piece::Pawn => {
					let f_incr:fn(i8,i8)->i8;
					let start_row:i8;

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
							if self.move_type(f_pos,t_pos) == MoveType::Capture {
								moves.push(t_pos);
							}
						}
					}
				},

				Piece::Knight => {
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
				},

				Piece::Bishop => generate_bishop(&mut moves,self,f_pos,7),

				Piece::Rook  => generate_rook(&mut moves,self,f_pos,7),

				Piece::Queen => {
					generate_bishop(&mut moves,self,f_pos,7);
					generate_rook(&mut moves,self,f_pos,7);
				},

				Piece::King => {
					generate_bishop(&mut moves,self,f_pos,1);
					generate_rook(&mut moves,self,f_pos,1);
					/* castling (TODO: cannot be used in currently in check) */
					if self.can_castle_left[self.player as usize] {
						if self.at(Pos::at(f_pos.col-1,f_pos.row).unwrap()).is_none() &&
						   self.at(Pos::at(f_pos.col-2,f_pos.row).unwrap()).is_none() {
								moves.push(Pos::at(f_pos.col-2,f_pos.row).unwrap());
						}
					}
					if self.can_castle_right[self.player as usize] {
						if self.at(Pos::at(f_pos.col+1,f_pos.row).unwrap()).is_none() &&
						   self.at(Pos::at(f_pos.col+2,f_pos.row).unwrap()).is_none() {
								moves.push(Pos::at(f_pos.col+2,f_pos.row).unwrap());
						}
					}
				}
			}
		}
		moves
	}
}
