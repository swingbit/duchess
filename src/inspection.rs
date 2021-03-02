use std::ops;

use crate::board::{Board,Pos,MoveType,Piece,Color};

impl Board {
	/// Check if the King of the given color is in check,
	/// by reversing an attack from the king's position
	/// using all types of movement
	pub fn is_king_in_check(&self, color: Color) -> bool {
		let king_pos = self.king_pos[color as usize];
		let f_incr:fn(i8,i8)->i8;

		/* check pawn attack */
		if color == Color::White {
			f_incr = ops::Add::add;
		} else {
			f_incr = ops::Sub::sub;
		}
		for i in [-1, 1].iter() {
			if let Some(pos) = Pos::at(king_pos.col+i, f_incr(king_pos.row,1)) {
				if let MoveType::Capture(Piece::Pawn) = self.move_type(king_pos,pos) {
					return true;
				}
			}
		}

		/* check an attack by other pieces */
		let mut moves = Vec::new();
		self.generate_bishop(&mut moves, king_pos, 7);
		if moves.iter().any(|&p| { 
			if let Some(tile) = self.at(p) {
				match tile.piece {
					Piece::Bishop|Piece::Queen|Piece::King => true,
					_ => false
				}
			} else { false }
		}) {
			return true
		}

		moves = Vec::new();
		self.generate_rook(&mut moves, king_pos, 7);
		if moves.iter().any(|&p| { 
			if let Some(tile) = self.at(p) {
				match tile.piece {
					Piece::Rook|Piece::Queen|Piece::King => true,
					_ => false
				}
			} else { false }
		}) {
			return true
		}

		moves = Vec::new();
		self.generate_knight(&mut moves, king_pos);
		if moves.iter().any(|&p| { 
			if let Some(tile) = self.at(p) {
				match tile.piece {
					Piece::Knight => true,
					_ => false
				}
			} else { false }
		}) {
			return true
		}
	
		false
	}

	pub fn move_type(&self, f_pos: Pos, t_pos: Pos) -> MoveType {
		if let Some(f_tile) = self.at(f_pos) {
			if let Some(t_tile) = self.at(t_pos) {
				if f_tile.color != t_tile.color {
					return MoveType::Capture(t_tile.piece);
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
						MoveType::Illegal | MoveType::Capture(_) => {
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
						if (t_pos.col - f_pos.col).abs() == 1 &&
								t_pos.row == f_incr(f_pos.row,1) &&
								t_tile.color != f_tile.color {
							return MoveType::Capture(t_tile.piece);
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
								return MoveType::Capture(t_tile.piece);
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
						/* castling (TODO: cannot be used if currently in check) */
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
}