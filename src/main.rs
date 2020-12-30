// use std::io::{self, BufRead};
// use std::cmp;
use std::ops;
use trees::{tr,Tree};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Color { Black, White }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Piece {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Tile {
	piece: Piece,	
	color: Color,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
struct Pos {
	// we use i8 instead of u8
	// to avoid overflows when generating positions that are possibly outside the board
	row: i8,
	col: i8,
}

impl Pos {
	fn at(col: i8, row:i8) -> Option<Pos> {
		if col < 0 || row < 0 || col > 7 || row > 7 {
			return None;
		}
		return Some(Pos {
			col: col,
			row: row,
	});
	}

	fn from_coord(x: char, y: char) -> Option<Pos> {
		if ! (x.is_ascii() && y.is_ascii()) {
			return None;
		}
		let x = x.to_ascii_uppercase();

		if ! (x.ge(&'A') && x.le(&'H') && y.ge(&'1') && y.le(&'8')) {
			return None;
		}

		return Some(Pos {
			col: ((x as u8) - b'A') as i8,
			row: ((y as u8) - b'1') as i8,
		});
	}

	fn to_coord(&self) -> (char,char) {
		return (
			((self.col as u8) + b'A') as char,
			((self.row as u8) + b'1') as char
		);
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MoveType { Illegal, Eat, Move }

trait Valuable {
	fn value(&self, for_color: Color) -> u32;
}

impl Valuable for Piece {
	fn value(&self, _for_color: Color) -> u32 {
		match &self {
			Piece::Pawn => 100,
			Piece::Knight => 350,
			Piece::Bishop => 355,
			Piece::Rook => 550,
			Piece::Queen => 1100,
			Piece::King => 1200,
		}
	}
}

impl Valuable for Tile {
	fn value(&self, for_color: Color) -> u32 {
		if self.color != for_color {
			return 0;
		}
		return self.piece.value(for_color);
	}
}

impl Valuable for Board {
	fn value(&self, for_color: Color) -> u32 {
		let mut v = 0;
		for tile in self.tiles.iter().flat_map(|r| r.iter()) {
			if let Some(tile) = tile {
				v += tile.value(for_color);
			}
		}
		return v;
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Board {
	tiles: [[Option<Tile>; 8]; 8],
}

impl Board {
	pub fn new() -> Board {
		let mut b = Board {
			tiles: [[None; 8]; 8],
		};

		b.tiles[0][0] = Some(Tile {piece: Piece::Rook, color: Color::White});
		b.tiles[0][1] = Some(Tile {piece: Piece::Knight, color: Color::White});
		b.tiles[0][2] = Some(Tile {piece: Piece::Bishop, color: Color::White});
		b.tiles[0][3] = Some(Tile {piece: Piece::Queen, color: Color::White});
		b.tiles[0][4] = Some(Tile {piece: Piece::King, color: Color::White});
		b.tiles[0][5] = Some(Tile {piece: Piece::Bishop, color: Color::White});
		b.tiles[0][6] = Some(Tile {piece: Piece::Knight, color: Color::White});
		b.tiles[0][7] = Some(Tile {piece: Piece::Rook, color: Color::White});
		b.tiles[1] = [Some(Tile {piece: Piece::Pawn, color: Color::White}); 8];

		b.tiles[7][0] = Some(Tile {piece: Piece::Rook, color: Color::Black});
		b.tiles[7][1] = Some(Tile {piece: Piece::Knight, color: Color::Black});
		b.tiles[7][2] = Some(Tile {piece: Piece::Bishop, color: Color::Black});
		b.tiles[7][3] = Some(Tile {piece: Piece::Queen, color: Color::Black});
		b.tiles[7][4] = Some(Tile {piece: Piece::King, color: Color::Black});
		b.tiles[7][5] = Some(Tile {piece: Piece::Bishop, color: Color::Black});
		b.tiles[7][6] = Some(Tile {piece: Piece::Knight, color: Color::Black});
		b.tiles[7][7] = Some(Tile {piece: Piece::Rook, color: Color::Black});
		b.tiles[6] = [Some(Tile {piece: Piece::Pawn, color: Color::Black}); 8];

		b
	}

	fn at(&self, pos: &Pos) -> &Option<Tile> {
		&self.tiles[pos.row as usize][pos.col as usize]
	}

	/* Checks whether a move is valid
		 Not needed for generated moves, only for human moves
		 Allows to "see through" a specified number of obstacles
	*/
	fn check_move(&self, f_pos: &Pos, t_pos: &Pos, max_obstacles: u8) -> MoveType {

		/* Checks possible moves from a given point in all directions */
		fn check_arm(b:&Board, f_pos:&Pos, t_pos:&Pos, max_obstacles: u8, 
					f_c:fn(i8,i8)->i8, f_r:fn(i8,i8)->i8) -> MoveType {
			let mut obstacles = 0;
			for i in 1..8 {
				if let Some(x_pos) = Pos::at(f_c(f_pos.col,i), f_r(f_pos.row,i)) {
					let move_type = b.move_type(f_pos,&x_pos);
					match &move_type {
						MoveType::Illegal | MoveType::Eat => {
							if obstacles == max_obstacles {
								if *t_pos == x_pos {
									return move_type;
								} else {
									return MoveType::Illegal;
								}
							};
							obstacles = obstacles + 1;
						},
						MoveType::Move => {
							if *t_pos == x_pos {
								return move_type;
							}
						}
					}
				} else { 
					return MoveType::Illegal; 
				}
			}
			return MoveType::Illegal;
		};

		fn check_bishop(b:&Board, f_pos:&Pos, t_pos:&Pos, max_obstacles:u8) -> MoveType {
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
			return MoveType::Illegal;
		}

		fn check_rook(b:&Board, f_pos:&Pos, t_pos:&Pos, max_obstacles:u8) -> MoveType {
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
			return MoveType::Illegal;
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
						/* Eat diagonally */
						if (t_pos.col -f_pos.col).abs() == 1 &&
								t_pos.row == f_incr(f_pos.row,1) &&
								t_tile.color != f_tile.color {
							return MoveType::Eat;
						}
					} else {
						/* forward by 1 or 2 */
						if t_pos.col == f_pos.col {
							if t_pos.row == f_incr(f_pos.row,1) ||
									(f_pos.row == start_row && t_pos.row == f_incr(start_row,2)) {
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
								return MoveType::Eat;
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
					let cb = check_bishop(self,f_pos,t_pos,max_obstacles);
					if cb != MoveType::Illegal {
						return cb;
					}
					return check_rook(self,f_pos,t_pos,max_obstacles);
				},
			}
		}
		return MoveType::Illegal;
	}

	fn move_type(&self, f_pos: &Pos, t_pos: &Pos) -> MoveType {
		if let Some(f_tile) = self.at(f_pos) {
			if let Some(t_tile) = self.at(t_pos) {
				if f_tile.color != t_tile.color {
					return MoveType::Eat;
				}
			} else {
				return MoveType::Move;
			}
		}
		return MoveType::Illegal;
	}
	
	fn generate_legal_moves(&self, f_pos: &Pos) -> Vec<Pos> {
		let mut moves = Vec::new();

		/* Generate possible moves from a given point in all directions */
		fn generate_arm(moves:&mut Vec<Pos>, b:&Board, f_pos:&Pos, max_len:i8, f_c:fn(i8,i8)->i8, f_r:fn(i8,i8)->i8) {
			for i in 1..max_len+1 {
				if let Some(t_pos) = Pos::at(f_c(f_pos.col,i), f_r(f_pos.row,i)) {
					match b.move_type(f_pos,&t_pos) {
						MoveType::Move => moves.push(t_pos),
						MoveType::Eat => { moves.push(t_pos); break; },
						MoveType::Illegal => break
					}
				} else { break; }
			}
		};

		/* Generate possible bishop moves from a given point */
		fn generate_bishop(moves:&mut Vec<Pos>, b:&Board, f_pos:&Pos, max_len:i8) {
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

		/* Generate possible rook moves from a given point */
		fn generate_rook(moves:&mut Vec<Pos>, b:&Board, f_pos:&Pos, max_len:i8) {
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
						if self.move_type(f_pos,&t_pos) == MoveType::Move {
							moves.push(t_pos);
						}
						if f_pos.row == start_row {
							/* forward by 2 */
							if let Some(t_pos) = Pos::at(f_pos.col, f_incr(f_pos.row,2)) {
								if self.move_type(f_pos,&t_pos) == MoveType::Move {
									moves.push(t_pos);
								}
							}
						}
					}
					/* Eat diagonally */
					for i in [-1, 1].iter() {
						if let Some(t_pos) = Pos::at(f_pos.col+i, f_incr(f_pos.row,1)) {
							if self.move_type(f_pos,&t_pos) == MoveType::Eat {
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
							if self.move_type(f_pos,&t_pos) != MoveType::Illegal {
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
				}
			}
		}
		return moves;
	}
}



fn main() {

	let b: Board = Board::new();
	let mut tree: Tree<Board> = tr(b);

	println!("The Board");
	for r in b.tiles.iter() {
		println!("{:?}", r);
	}
	println!("Board value for white = {}", &b.value(Color::White));

	let pos = Pos::from_coord('G','8').unwrap();
	println!("G8 = {:?}", &pos);
	let (x,y) = &pos.to_coord();
	println!("{:?} = [{},{}]", &pos, x, y);

	for c in 0..8 {
		for r in 0..8 {
			let f_pos = Pos::at(c,r).unwrap();
			if let Some(f_tile) = b.at(&f_pos) {
				if f_tile.color == Color::White {
					print!("[{},{}] ({:?}): ",c,r,f_tile.piece);
					println!("{:?}", b.generate_legal_moves(&f_pos));
				}
			}
		}
	}

}

