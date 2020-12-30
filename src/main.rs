// use std::io::{self, BufRead};
use std::cmp;
// use std::fmt::Display;
// use trees::{tr,Tree};

trait Valuable {
	fn value(&self, for_color: Color) -> u32;
}

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
enum MoveType { Invalid, Eat, Move }

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Tile {
	piece: Piece,	
	color: Color,
}

impl Valuable for Tile {
	fn value(&self, for_color: Color) -> u32 {
		if self.color != for_color {
			return 0;
		}
		return self.piece.value(for_color);
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

	fn tile_at_pos(&self, pos: &Pos) -> Option<Tile> {
		self.tiles[pos.row as usize][pos.col as usize]
	}
	fn tile_at(&self, col: i8, row: i8) -> Option<Tile> {
		self.tiles[row as usize][col as usize]
	}

	fn move_type(&self, f_pos: &Pos, t_pos: &Pos) -> MoveType {
		// TODO: no move is valid if it makes its own king under check
		// But it needs to be checked on the board with the move already applied
		if let Some(f_tile) = &self.tile_at_pos(f_pos) {
			match f_tile.piece {
				Piece::Pawn => {
					match f_tile.color {
						Color::White => {
							if (t_pos.row == f_pos.row+1 || (f_pos.row == 1 && t_pos.row == 3)) && t_pos.col == f_pos.col && self.tile_at_pos(t_pos).is_none() {
								return MoveType::Move;
							}
							if let Some(t_tile) = self.tile_at_pos(t_pos) {
								if t_pos.row == f_pos.row+1 && (t_pos.col == f_pos.col-1 || t_pos.col == f_pos.col+1) && t_tile.color != f_tile.color {
									return MoveType::Eat;
								}
							}
						},
						Color::Black => {
							if (t_pos.row == f_pos.row-1 || (f_pos.row == 6 && t_pos.row == 4)) && t_pos.col == f_pos.col && self.tile_at_pos(t_pos).is_none() {
								return MoveType::Move;
							}
							if let Some(t_tile) = self.tile_at_pos(t_pos) {
								if t_pos.row == f_pos.row-1 && (t_pos.col == f_pos.col-1 || t_pos.col == f_pos.col+1) && t_tile.color != f_tile.color {
									return MoveType::Eat;
								}
							}
						}
					}
				},

				Piece::Knight => {
					if ((f_pos.row-t_pos.row).abs() == 1 && (f_pos.col-t_pos.col).abs() == 2) ||
						 ((f_pos.row-t_pos.row).abs() == 2 && (f_pos.col-t_pos.col).abs() == 1) {
						if let Some(t_tile)	= &self.tile_at_pos(t_pos) {
							if t_tile.color != f_tile.color {
								return MoveType::Eat;
							}
						} else {
							return MoveType::Move;
						}
					}
				},

				Piece::Bishop | Piece::Queen => {

				},

				Piece::Rook | Piece::Queen => {

				},

				Piece::Rook | Piece::Queen => {

				},
				_ => ()

			}
		}
		return MoveType::Invalid;
	}

	fn generate_valid_moves(&self, f_pos: &Pos) -> Vec<Pos> {
		let mut moves = Vec::new();
		if let Some(f_tile) = &self.tile_at_pos(f_pos) {
			match f_tile.piece {
				Piece::Pawn => {
					match f_tile.color {
						Color::White => {
							if f_pos.row == 1 {
								if let Some(t_pos) = Pos::at(f_pos.col, f_pos.row+2) {
									if self.move_type(f_pos,&t_pos) == MoveType::Move {
										moves.push(t_pos);
									}
								}
							}
							for i in [-1, 0, 1].iter() {
								if let Some(t_pos) = Pos::at(f_pos.col+i, f_pos.row+1) {
									if self.move_type(f_pos,&t_pos) != MoveType::Invalid {
										moves.push(t_pos);
									}
								}
							}
						}
						Color::Black => {
							if f_pos.row == 6 {
								if let Some(t_pos) = Pos::at(f_pos.col, f_pos.row-2) {
									if self.move_type(f_pos,&t_pos) == MoveType::Move {
										moves.push(t_pos);
									}
								}
							}
							for i in [-1, 0, 1].iter() {
								if let Some(t_pos) = Pos::at(f_pos.col+i, f_pos.row-1) {
									if self.move_type(f_pos,&t_pos) != MoveType::Invalid {
										moves.push(t_pos);
									}
								}
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
							if self.move_type(f_pos,&t_pos) != MoveType::Invalid {
								moves.push(t_pos);
							}
						}
					}
				},

				Piece::Bishop | Piece::Queen => {
					/* north-west */
					for i in 1..8 {
						if let Some(t_pos) = Pos::at(f_pos.col-i, f_pos.row+i) {
							match self.move_type(f_pos,&t_pos) {
								MoveType::Move | MoveType::Eat => moves.push(t_pos),
								MoveType::Eat | MoveType::Invalid => break
							}
						} else { break; }
					}
					/* north-east */
					for i in 1..8 {
						if let Some(t_pos) = Pos::at(f_pos.col+i, f_pos.row+i) {
							match self.move_type(f_pos,&t_pos) {
								MoveType::Move | MoveType::Eat => moves.push(t_pos),
								MoveType::Eat | MoveType::Invalid => break
							}
						} else { break; }
					}
					/* south-east */
					for i in 1..8 {
						if let Some(t_pos) = Pos::at(f_pos.col+i, f_pos.row-i) {
							match self.move_type(f_pos,&t_pos) {
								MoveType::Move | MoveType::Eat => moves.push(t_pos),
								MoveType::Eat | MoveType::Invalid => break
							}
						} else { break; }
					}
					/* south-west */
					for i in 1..8 {
						if let Some(t_pos) = Pos::at(f_pos.col-i, f_pos.row-i) {
							match self.move_type(f_pos,&t_pos) {
								MoveType::Move | MoveType::Eat => moves.push(t_pos),
								MoveType::Eat | MoveType::Invalid => break
							}
						} else { break; }
					}
				},

				Piece::Rook | Piece::Queen => {
					/* north */
					for i in 1..8 {
						if let Some(t_pos) = Pos::at(f_pos.col, f_pos.row+i) {
							match self.move_type(f_pos,&t_pos) {
								MoveType::Move | MoveType::Eat => moves.push(t_pos),
								MoveType::Eat | MoveType::Invalid => break
							}
						} else { break; }
					}
					/* east */
					for i in 1..8 {
						if let Some(t_pos) = Pos::at(f_pos.col+i, f_pos.row) {
							match self.move_type(f_pos,&t_pos) {
								MoveType::Move | MoveType::Eat => moves.push(t_pos),
								MoveType::Eat | MoveType::Invalid => break
							}
						} else { break; }
					}
					/* south */
					for i in 1..8 {
						if let Some(t_pos) = Pos::at(f_pos.col, f_pos.row-i) {
							match self.move_type(f_pos,&t_pos) {
								MoveType::Move | MoveType::Eat => moves.push(t_pos),
								MoveType::Eat | MoveType::Invalid => break
							}
						} else { break; }
					}
					/* west */
					for i in 1..8 {
						if let Some(t_pos) = Pos::at(f_pos.col-i, f_pos.row) {
							match self.move_type(f_pos,&t_pos) {
								MoveType::Move | MoveType::Eat => moves.push(t_pos),
								MoveType::Eat | MoveType::Invalid => break
							}
						} else { break; }
					}
				},

				Piece::King => {
					for c in [-1, 0, 1].iter() {
						for r in [-1, 0, 1].iter() {
							if *c != 0 || *r != 0 {
								if let Some(t_pos) = Pos::at(f_pos.col+*c, f_pos.row+*r) {
									if self.move_type(f_pos,&t_pos) != MoveType::Invalid {
										moves.push(t_pos);
									}
								}
							}
						}
					}
		},
				_ => ()
			}
		}
		return moves;
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

#[derive(Debug)]
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

fn main() {

	// let mut tree: Tree<Board> = tr(Board::new());
	let mut b: Board = Board::new();

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
			if let Some(f_tile) = b.tile_at_pos(&f_pos) {
				if f_tile.color == Color::White {
					print!("[{},{}] ({:?}): ",c,r,f_tile.piece);
					println!("{:?}", b.generate_valid_moves(&f_pos));
				}
			}
		}
	}

}

