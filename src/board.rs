use std::fmt;
use std::str::FromStr;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct ParsePosError;
impl fmt::Display for ParsePosError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			"invalid position".fmt(f)
	}
}

impl Error for ParsePosError {
	fn description(&self) -> &str {
			"invalid position"
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Color { Black, White }
impl Color {
	fn swap(&self) -> Color {
		match self {
			Color::White => Color::Black,
			Color::Black => Color::White
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
	Pawn,
	Knight,
	Bishop,
	Rook,
	Queen,
	King,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tile {
	pub piece: Piece,	
	pub color: Color,
}

#[derive(Debug,Copy,Clone,PartialEq,Eq)]
pub struct Pos {
	// we use i8 instead of u8
	// to avoid overflows when generating positions that are possibly outside the board
	pub row: i8,
	pub col: i8,
}

impl fmt::Display for Pos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (x,y) = self.to_coord();
		write!(f, "{}{}", x,y)
	}
}

impl FromStr for Pos {
	type Err = ParsePosError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
			if let Some(pos) = Self::from_coord(s) {
				return Ok(pos);
			} else {
				return Err(ParsePosError);
			}
	}
}

impl Pos {
	pub fn at(col: i8, row:i8) -> Option<Pos> {
		if col < 0 || row < 0 || col > 7 || row > 7 {
			return None;
		}
		return Some(Pos {
			col: col,
			row: row,
		});
	}

	fn from_coord(s: &str) -> Option<Pos> {
		if s.len() != 2 {
			return None;
		}
		let x = s.chars().nth(0).unwrap();
		let y = s.chars().nth(1).unwrap();

		if ! (x.is_ascii() && y.is_ascii()) {
			return None;
		}
		let x = x.to_ascii_lowercase();

		if ! (x.ge(&'a') && x.le(&'h') && y.ge(&'1') && y.le(&'8')) {
			return None;
		}

		return Some(Pos {
			col: ((x as u8) - b'a') as i8,
			row: ((y as u8) - b'1') as i8,
		});
	}

	fn to_coord(&self) -> (char,char) {
		( ((self.col as u8) + b'a') as char, ((self.row as u8) + b'1') as char )
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveType { Illegal, Capture, Move }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Board {
	pub tiles: [[Option<Tile>; 8]; 8],
	// pub value: i32,
	pub player: Color,
}

impl Board {
	pub fn new(player: Color) -> Board {
		let mut b = Board {
			tiles: [[None; 8]; 8],
			// value: 0,
			player,
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

	pub fn at(&self, pos: &Pos) -> &Option<Tile> {
		&self.tiles[pos.row as usize][pos.col as usize]
	}

	pub fn clone_apply_move(&self, f_pos: &Pos, t_pos: &Pos) -> Self {
		let mut b = self.clone();
		b.tiles[t_pos.row as usize][t_pos.col as usize] = self.tiles[f_pos.row as usize][f_pos.col as usize];
		b.tiles[f_pos.row as usize][f_pos.col as usize] = None;
		b.player = b.player.swap();
		b
	}

}


pub trait Valuable {
	fn value(&self) -> i32;
}

impl Valuable for Piece {
	fn value(&self) -> i32 {
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
	fn value(&self) -> i32 {
		match self.color {
			Color::Black => 
				return -self.piece.value(),
			Color::White => 
				return self.piece.value(),
		}
	}
}

impl Valuable for Board {
	fn value(&self) -> i32 {
		let mut v = 0;
		for tile in self.tiles.iter().flat_map(|r| r.iter()) {
			if let Some(tile) = tile {
				v += tile.value();
			}
		}
		return v;
	}
}

