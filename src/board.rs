use std::error::Error;
use std::fmt;
use std::str::FromStr;
use std::cell::Cell;

use crate::evaluation::{Value};

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
pub enum Color {
	Black = 0,
	White = 1,
}
impl Color {
	#[inline]
	pub fn swap(&self) -> Color {
		match self {
			Color::White => Color::Black,
			Color::Black => Color::White,
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
	Pawn = 0,
	Knight = 1,
	Bishop = 2,
	Rook = 3,
	Queen = 4,
	King = 5,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tile {
	pub piece: Piece,
	pub color: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Pos {
	// we use i8 instead of u8
	// to avoid overflows when generating positions that are possibly outside the board
	pub row: i8,
	pub col: i8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Move {
	pub f_pos: Pos,
	pub t_pos: Pos,
}

impl fmt::Display for Pos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (x, y) = self.to_coord();
		write!(f, "{}{}", x, y)
	}
}

impl FromStr for Pos {
	type Err = ParsePosError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some(pos) = Self::from_coord(s) {
			Ok(pos)
		} else {
			Err(ParsePosError)
		}
	}
}

impl fmt::Display for Move {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}{}", self.f_pos, self.t_pos)
	}
}

impl FromStr for Move {
	type Err = ParsePosError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let f_pos = s[0..2].parse::<Pos>()?;
		let t_pos = s[2..4].parse::<Pos>()?;
		Ok(Move {f_pos, t_pos})
	}
}


impl Pos {
	pub fn at(col: i8, row: i8) -> Option<Pos> {
		if col < 0 || row < 0 || col > 7 || row > 7 {
			None
		} else {
			Some(Pos { col: col, row: row })
		}
	}

	pub fn from_coord(s: &str) -> Option<Pos> {
		if s.len() != 2 {
			return None;
		}
		let x = s.chars().nth(0).unwrap();
		let y = s.chars().nth(1).unwrap();

		if !(x.is_ascii() && y.is_ascii()) {
			return None;
		}
		let x = x.to_ascii_lowercase();

		if !(x.ge(&'a') && x.le(&'h') && y.ge(&'1') && y.le(&'8')) {
			return None;
		}

		Some(Pos {
			col: ((x as u8) - b'a') as i8,
			row: ((y as u8) - b'1') as i8,
		})
	}

	fn to_coord(&self) -> (char, char) {
		(
			((self.col as u8) + b'a') as char,
			((self.row as u8) + b'1') as char,
		)
	}
}


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveType {
	Illegal,
	Capture,
	Move,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
	pub tiles: [[Option<Tile>; 8]; 8],
	pub player: Color,
	pub king_pos: [Pos; 2],
	pub stored_value: Cell<Option<Value>>,
}

impl Board {
	pub fn new(player: Color) -> Board {
		let mut b = Board {
			tiles: [[None; 8]; 8],
			player,
			king_pos: [Pos::at(4,7).unwrap(), Pos::at(4,0).unwrap()],
			stored_value: Cell::default(),
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

	#[inline]
	pub fn at(&self, pos: Pos) -> &Option<Tile> {
		&self.tiles[pos.row as usize][pos.col as usize]
	}

	pub fn clone_apply_move(&self, f_pos: Pos, t_pos: Pos) -> Board {
		let mut b = self.clone();
		b.tiles[t_pos.row as usize][t_pos.col as usize] = self.tiles[f_pos.row as usize][f_pos.col as usize];
		b.tiles[f_pos.row as usize][f_pos.col as usize] = None;
		b.player = b.player.swap();
		let t = b.at(t_pos).unwrap();
		if t.piece == Piece::King {
			b.king_pos[t.color as usize] = t_pos;
		}
		b.stored_value = Cell::default();
		b
	}
}

#[cfg(test)]
mod tests {
	use crate::board::Pos;
	#[test]
	pub fn test_parse_pos() {
		let coords_in = "G8";
		let pos = coords_in.parse::<Pos>().expect("legal");
		let coords_out = pos.to_string();
		assert_eq!(coords_in.to_ascii_lowercase(), coords_out);
	}
}
