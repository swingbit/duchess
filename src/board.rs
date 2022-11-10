use std::fmt;
use std::str::FromStr;
use std::cell::Cell;

use crate::evaluation::{Value};

#[derive(Clone, Debug)]
pub struct ParseError;
impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		"invalid position".fmt(f)
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

	pub fn from_char(c: char) -> Option<Color> {
		match c {
			'b' | 'B' => Some(Color::Black),
			'w' | 'W' => Some(Color::White),
			_ => None
		}
	}

	pub fn to_char(&self) -> char {
		match self {
			Color::White => 'w',
			Color::Black => 'b',
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

impl Piece {
	pub fn as_char(self) -> char {
		match self {
				Piece::Pawn => 'p',
				Piece::Knight => 'n',
				Piece::Bishop => 'b',
				Piece::Rook => 'r',
				Piece::Queen => 'q',
				Piece::King => 'k'
		}
	}

	pub fn from_char(c: char) -> Result<Piece, ParseError> {
		match c.to_lowercase().next() {
			Some('p') => Ok(Piece::Pawn),
			Some('n') => Ok(Piece::Knight),
			Some('b') => Ok(Piece::Bishop),
			Some('r') => Ok(Piece::Rook),
			Some('k') => Ok(Piece::King),
			Some('q') => Ok(Piece::Queen),
			_ => Err(ParseError)
		}
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Tile {
	pub piece: Piece,
	pub color: Color,
}

impl Tile {
	pub fn from_char(c: char) -> Option<Tile> {
		match c {
			'p' => Some(Tile { piece: Piece::Pawn, color: Color::Black}),
			'n' => Some(Tile { piece: Piece::Knight, color: Color::Black}),
			'b' => Some(Tile { piece: Piece::Bishop, color: Color::Black}),
			'r' => Some(Tile { piece: Piece::Rook, color: Color::Black}),
			'k' => Some(Tile { piece: Piece::King, color: Color::Black}),
			'q' => Some(Tile { piece: Piece::Queen, color: Color::Black}),
			'P' => Some(Tile { piece: Piece::Pawn, color: Color::White}),
			'N' => Some(Tile { piece: Piece::Knight, color: Color::White}),
			'B' => Some(Tile { piece: Piece::Bishop, color: Color::White}),
			'R' => Some(Tile { piece: Piece::Rook, color: Color::White}),
			'K' => Some(Tile { piece: Piece::King, color: Color::White}),
			'Q' => Some(Tile { piece: Piece::Queen, color: Color::White}),
			_ => None
		}
	}

	pub fn as_char(&self) -> char {
		match self {
			Tile { piece: Piece::Pawn, color: Color::Black} => 'p',
			Tile { piece: Piece::Knight, color: Color::Black} => 'n',
			Tile { piece: Piece::Bishop, color: Color::Black} => 'b',
			Tile { piece: Piece::Rook, color: Color::Black} => 'r',
			Tile { piece: Piece::King, color: Color::Black} => 'k',
			Tile { piece: Piece::Queen, color: Color::Black} => 'q',
			Tile { piece: Piece::Pawn, color: Color::White} => 'P',
			Tile { piece: Piece::Knight, color: Color::White} => 'N',
			Tile { piece: Piece::Bishop, color: Color::White} => 'B',
			Tile { piece: Piece::Rook, color: Color::White} => 'R',
			Tile { piece: Piece::King, color: Color::White} => 'K',
			Tile { piece: Piece::Queen, color: Color::White} => 'Q'
		}
	}
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
	pub promotion: Option<Piece>
}

impl fmt::Display for Pos {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let (x, y) = self.to_coord();
		write!(f, "{}{}", x, y)
	}
}

impl FromStr for Pos {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some(pos) = Self::from_coord(s) {
			Ok(pos)
		} else {
			Err(ParseError)
		}
	}
}

impl fmt::Display for Move {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let mut s = write!(f, "{}{}", self.f_pos, self.t_pos);
		if let Some(p) = self.promotion {
			if p != Piece::Pawn {
					s = write!(f, "{}", p.as_char());
			}
		}
		s
	}
}

impl FromStr for Move {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let f_pos = s[0..2].parse::<Pos>()?;
		let t_pos = s[2..4].parse::<Pos>()?;
		if s.len() == 5 {
			let promotion = Piece::from_char(s.chars().nth(4).unwrap())?;
			return Ok(Move {f_pos, t_pos,promotion:Some(promotion)});
		} else {
			return Ok(Move {f_pos, t_pos,promotion:None});
		}
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
	Capture(Piece),
	Move,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
	pub tiles: [[Option<Tile>; 8]; 8],
	pub player: Color,
	pub king_pos: [Pos; 2],
	/*	left and right in the castling always refers to 
			looking at the board from the white's position */
	pub can_castle_left: [bool; 2],
	pub can_castle_right: [bool; 2],
	pub stored_value: Cell<Option<Value>>,
}

impl Board {
	pub fn from_fen(s: &str) -> Result<Board, ParseError> {
		fn parse_ranks(s: Option<&str>) -> Result <([[Option<Tile>; 8]; 8], [Pos; 2]), ParseError> {
			let s = s.ok_or(ParseError)?;
			let ranks = s.split('/');

			let mut tiles:[[Option<Tile>; 8]; 8] = [[None; 8]; 8];
			let mut king_pos:[Option<Pos>; 2] = [None; 2];

			let mut rowcount:usize = 0;
			for rank in ranks {
				let mut colcount = 0; 

				for x in rank.chars() {
					if let Some(blanks) = char::to_digit(x, 10) {
						colcount += blanks as usize
					} else {
						let t:Option<Tile> = Tile::from_char(x);
						if t.is_none() {
							return Err(ParseError)
						}
						tiles[7-rowcount][colcount] = t;
						colcount += 1;
						if t.unwrap().piece == Piece::King {
							king_pos[t.unwrap().color as usize] = Pos::at(colcount as i8,7-rowcount as i8);
						}
					}
				}
				if colcount != 8 {
					return Err(ParseError)
				}
				rowcount += 1;
			}
			if rowcount != 8 {
				return Err(ParseError)
			}
			let king_pos = if let [Some(a), Some(b)] = king_pos {
				[a,b]
			} else {
				return Err(ParseError)
			};

			Ok((tiles,king_pos))
		}

		fn parse_player(s: Option<&str>) -> Result <Color, ParseError> {
			let s = s.ok_or(ParseError)?;

			if s.len() != 1 {
				return Err(ParseError)
			}

			if let Some(c) = s.chars().nth(0) {
				if let Some(color) = Color::from_char(c) {
					return Ok(color)
				}
			}
			return Err(ParseError)
		}

		fn parse_castle(s: Option<&str>) -> Result <([bool; 2], [bool; 2]), ParseError> {
			let s = s.ok_or(ParseError)?;

			let mut can_parse_left = [false ;2];
			let mut can_parse_right = [false ;2];

			if s.find('K').is_some() {
				can_parse_right[Color::White as usize] = true;
			}
			if s.find('k').is_some() {
				can_parse_right[Color::Black as usize] = true;
			}
			if s.find('Q').is_some() {
				can_parse_left[Color::White as usize] = true;
			}
			if s.find('q').is_some() {
				can_parse_left[Color::Black as usize] = true;
			}

			Ok((can_parse_left, can_parse_right))
		}
		
		let mut split = s.split_whitespace();
		let (tiles, king_pos) = parse_ranks(split.next())?;
		let player = parse_player(split.next())?;
		let (can_castle_left, can_castle_right) = parse_castle(split.next())?;

		// TODO, parse the en-passant and move counts

		let b = Board {
			tiles,
			player,
			king_pos,
			can_castle_left,
			can_castle_right,
			stored_value: Cell::default(),
		};

		Ok(b)
	}

	pub fn to_fen(&self) -> String {
		let mut s = String::new();

		// ranks
		for r in (0..8).rev() {
			let mut blanks = 0;
			for c in 0..8 {
				let pos = Pos::at(c,r).unwrap();
				if let Some(tile) = self.at(pos) {
					if blanks > 0 {
						s.push(char::from_digit(blanks, 10).unwrap());
						blanks = 0;
					}
					s.push(tile.as_char());
				} else {
					blanks += 1;
				}
			}
			if blanks > 0 {
				s.push(char::from_digit(blanks, 10).unwrap());
			}
			if r > 0 {
				s.push('/');
			}
		}

		s.push(' ');

		// player
		s.push(self.player.to_char());

		s.push(' ');

		// castle
		if self.can_castle_left == [false; 2] && self.can_castle_right == [false; 2] {
			s.push('-');
		} else {
			if self.can_castle_right[Color::White as usize] {
				s.push('K');
			}
			if self.can_castle_left[Color::White as usize] {
				s.push('Q');
			}
			if self.can_castle_right[Color::Black as usize] {
				s.push('k');
			}
			if self.can_castle_left[Color::Black as usize] {
				s.push('q');
			}
		}

		// TODO: implement the rest
		s.push_str(" - 0 0");

		s
	}
	
	pub fn new(player: Color) -> Board {
		let fen = format!("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR {} KQkq - 0 1", player.to_char());
		Self::from_fen(&fen).unwrap()
	}

	#[inline]
	pub fn at(&self, pos: Pos) -> &Option<Tile> {
		&self.tiles[pos.row as usize][pos.col as usize]
	}

	/// Clone current board and apply given move.
	/// The move is assumed to be valid
	pub fn clone_apply_move(&self, mv: &Move) -> Board {
		let mut b = self.clone();
		b.player = b.player.swap();
		b.tiles[mv.t_pos.row as usize][mv.t_pos.col as usize] = b.tiles[mv.f_pos.row as usize][mv.f_pos.col as usize];
		b.tiles[mv.f_pos.row as usize][mv.f_pos.col as usize] = None;
		let t = b.at(mv.t_pos).unwrap();

		/* Castling: move Rook as well */
		if t.piece == Piece::King && (mv.f_pos.col - mv.t_pos.col).abs() == 2 {
			match t.color {
				Color::Black => {
					if mv.t_pos == Pos::at(6,7).unwrap() {
						b.tiles[7][5] = b.tiles[7][7];
						b.tiles[7][7] = None;
					} else if mv.t_pos == Pos::at(2,7).unwrap() {
						b.tiles[7][3] = b.tiles[7][0];
						b.tiles[7][0] = None;
					}
				},
				Color::White => {
					if mv.t_pos == Pos::at(6,0).unwrap() {
						b.tiles[0][5] = b.tiles[0][7];
						b.tiles[0][7] = None;
					} else if mv.t_pos == Pos::at(2,0).unwrap() {
						b.tiles[0][3] = b.tiles[0][0];
						b.tiles[0][0] = None;
					}
				}
			}
		}

		/* King moved */
		if t.piece == Piece::King {
			b.king_pos[t.color as usize] = mv.t_pos;
			/* disallow castling */
			b.can_castle_left[t.color as usize] = false;
			b.can_castle_right[t.color as usize] = false;
		}

		// /* Rook moved: disallow castling on its side */
		if t.piece == Piece::Rook {
			match t.color {
				Color::Black => {
					if mv.f_pos == Pos::at(0,7).unwrap() {
						b.can_castle_left[t.color as usize] = false;
					}
					if mv.f_pos == Pos::at(7,7).unwrap() {
						b.can_castle_right[t.color as usize] = false;
					}
				},
				Color::White => {
					if mv.f_pos == Pos::at(0,0).unwrap() {
						b.can_castle_left[t.color as usize] = false;
					}
					if mv.f_pos == Pos::at(7,0).unwrap() {
						b.can_castle_right[t.color as usize] = false;
					}
				}
			}
		}
		b.stored_value = Cell::default();

		b
	}
	
	#[allow(dead_code)]
	pub fn as_ascii(&self) -> String {
		let mut s = String::new();
		s.push_str("+---------------+\n");
		for r in (0..8).rev() {
			for c in 0..8 {
				let pos = Pos::at(c,r).unwrap();
				let mut p = ' ';
				if let Some(tile) = self.at(pos) {
					p = tile.as_char();
				}
				s.push_str(&format!("|{}",p));
			}
			s.push_str("|\n");
			s.push_str("+---------------+\n");
		}
		s.push_str("\n");
		s.push_str(&format!("value: {:?}\n",self.stored_value));
		s.push_str(&format!("player: {:?}\n",self.player));
		s.push_str(&format!("king_pos: {:?}\n",self.king_pos));
		s.push_str(&format!("can_castle_left: {:?}\n",self.can_castle_left));
		s.push_str(&format!("can_castle_right: {:?}\n",self.can_castle_right));
		s.push_str(&format!("check: {}\n",self.is_king_in_check(self.player)));
		s
	}
}

#[cfg(test)]
mod tests {
	use crate::board::{Pos,Move,Board};
	#[test]
	pub fn test_parse_pos() {
		let coords_in = "G8";
		let pos = coords_in.parse::<Pos>().expect("legal");
		let coords_out = pos.to_string();
		assert_eq!(coords_in.to_ascii_lowercase(), coords_out);
	}

	#[test]
	pub fn test_parse_move() {
		let mv_in = "e7e8";
		let mv = mv_in.parse::<Move>().expect("legal");
		let mv_out = mv.to_string();
		assert_eq!(mv_in.to_ascii_lowercase(), mv_out);
	}
	#[test]
	pub fn test_parse_move_promotion() {
		let mv_in = "e7e8q";
		let mv = mv_in.parse::<Move>().expect("legal");
		let mv_out = mv.to_string();
		assert_eq!(mv_in.to_ascii_lowercase(), mv_out);
	}

	#[test]
	pub fn test_fen1() {
		let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0";
		let b = Board::from_fen(fen).unwrap();
		assert_eq!(fen, b.to_fen());
	}
}
