use crate::board::{Board, Color, Piece, Pos, Tile};

pub type Value = i16;

/* 
 * Simplified Evaluation Function: https://www.chessprogramming.org/Simplified_Evaluation_Function
 * Please note that the values presented here have been designed specifically to compensate for the lack of any other chess knowledge, and NOT for being supplemented by it.
*/

/* Pawn positional bonus */
const MG_POS_BONUS_WHITE_PAWN: [[Value; 8]; 8] = [
	[  0,   0,   0,   0,   0,   0,   0,   0],
	[  5,  10,  10, -20, -20,  10,  10,   5],
	[  5,  -5, -10,   0,   0, -10,  -5,  -5],
	[  0,   0,   0,  20,  20,   0,   0,   0],
	[  5,   5,  10,  25,  25,  10,   5,   5],
	[ 10,  10,  20,  30,  30,  20,  10,  10],
	[ 50,  50,  50,  50,  50,  50,  50,  50],
	[  0,   0,   0,   0,   0,   0,   0,   0],
];

const MG_POS_BONUS_BLACK_PAWN: [[Value; 8]; 8] = [
	MG_POS_BONUS_WHITE_PAWN[7],
	MG_POS_BONUS_WHITE_PAWN[6],
	MG_POS_BONUS_WHITE_PAWN[5],
	MG_POS_BONUS_WHITE_PAWN[4],
	MG_POS_BONUS_WHITE_PAWN[3],
	MG_POS_BONUS_WHITE_PAWN[2],
	MG_POS_BONUS_WHITE_PAWN[1],
	MG_POS_BONUS_WHITE_PAWN[0],
];

/* Knight positional bonus */
const MG_POS_BONUS_WHITE_KNIGHT: [[Value; 8]; 8] = [
	[-50, -40, -30, -30, -30, -30, -40, -50],
	[-40, -20,   0,   0,   0,   0, -20, -40],
	[-30,   0,  10,  15,  15,  10,   0, -30],
	[-30,   5,  15,  20,  20,  15,   5, -30],
	[-30,   0,  15,  20,  20,  15,   0, -30],
	[-30,   5,  10,  15,  15,  10,   5, -30],
	[-40, -20,   0,   5,   5,   0, -20, -40],
	[-50, -40, -30, -30, -30, -30, -40, -50],
];

const MG_POS_BONUS_BLACK_KNIGHT: [[Value; 8]; 8] = MG_POS_BONUS_WHITE_KNIGHT;

/* Bishop positional bonus */
const MG_POS_BONUS_WHITE_BISHOP: [[Value; 8]; 8] = [
	[-20, -10, -10, -10, -10, -10, -10, -20],
	[-10,   5,   0,   0,   0,   0,   5, -10],
	[-10,  10,  10,  10,  10,  10,  10, -10],
	[-10,   0,  10,  10,  10,  10,   0, -10],
	[-10,   5,   5,  10,  10,   5,   5, -10],
	[-10,   0,   5,  10,  10,   5,   0, -10],
	[-10,   0,   0,   0,   0,   0,   0, -10],
	[-20, -10, -10, -10, -10, -10, -10, -20],
];

const MG_POS_BONUS_BLACK_BISHOP: [[Value; 8]; 8] = [
	MG_POS_BONUS_WHITE_BISHOP[7],
	MG_POS_BONUS_WHITE_BISHOP[6],
	MG_POS_BONUS_WHITE_BISHOP[5],
	MG_POS_BONUS_WHITE_BISHOP[4],
	MG_POS_BONUS_WHITE_BISHOP[3],
	MG_POS_BONUS_WHITE_BISHOP[2],
	MG_POS_BONUS_WHITE_BISHOP[1],
	MG_POS_BONUS_WHITE_BISHOP[0],
];

/* Rook positional bonus */
const MG_POS_BONUS_WHITE_ROOK: [[Value; 8]; 8] = [
	[  0,   0,   0,   5,   5,   0,   0,   0],
	[ -5,   0,   0,   0,   0,   0,   0,  -5],
	[ -5,   0,   0,   0,   0,   0,   0,  -5],
	[ -5,   0,   0,   0,   0,   0,   0,  -5],
	[ -5,   0,   0,   0,   0,   0,   0,  -5],
	[ -5,   0,   0,   0,   0,   0,   0,  -5],
	[  5,  10,  10,  10,  10,  10,  10,   5],
	[  0,   0,   0,   0,   0,   0,   0,   0],
];

const MG_POS_BONUS_BLACK_ROOK: [[Value; 8]; 8] = [
	MG_POS_BONUS_WHITE_ROOK[7],
	MG_POS_BONUS_WHITE_ROOK[6],
	MG_POS_BONUS_WHITE_ROOK[5],
	MG_POS_BONUS_WHITE_ROOK[4],
	MG_POS_BONUS_WHITE_ROOK[3],
	MG_POS_BONUS_WHITE_ROOK[2],
	MG_POS_BONUS_WHITE_ROOK[1],
	MG_POS_BONUS_WHITE_ROOK[0],
];

/* Queen positional bonus */
const MG_POS_BONUS_WHITE_QUEEN: [[Value; 8]; 8] = [
	[-20, -10, -10,  -5,  -5, -10, -10, -20],
	[-10,   0,   5,   0,   0,   0,   0, -10],
	[-10,   5,   5,   5,   5,   5,   0, -10],
	[  0,   0,   5,   5,   5,   5,   0,  -5],
	[ -5,   0,   5,   5,   5,   5,   0,  -5],
	[-10,   0,   5,   5,   5,   5,   0, -10],
	[-10,   0,   0,   0,   0,   0,   0, -10],
	[-20, -10, -10,  -5,  -5, -10, -10, -20],
];

const MG_POS_BONUS_BLACK_QUEEN: [[Value; 8]; 8] = [
	MG_POS_BONUS_WHITE_QUEEN[7],
	MG_POS_BONUS_WHITE_QUEEN[6],
	MG_POS_BONUS_WHITE_QUEEN[5],
	MG_POS_BONUS_WHITE_QUEEN[4],
	MG_POS_BONUS_WHITE_QUEEN[3],
	MG_POS_BONUS_WHITE_QUEEN[2],
	MG_POS_BONUS_WHITE_QUEEN[1],
	MG_POS_BONUS_WHITE_QUEEN[0],
];

/* King positional bonus */
const MG_POS_BONUS_WHITE_KING: [[Value; 8]; 8] = [
	[ 20,  30,  10,   0,   0,  10,  30,  20],
	[ 20,  20,   0,   0,   0,   0,  20,  20],
	[-10, -20, -20, -20, -20, -20, -20, -10],
	[-20, -30, -30, -40, -40, -30, -30, -20],
	[-30, -40, -40, -50, -50, -40, -40, -30],
	[-30, -40, -40, -50, -50, -40, -40, -30],
	[-30, -40, -40, -50, -50, -40, -40, -30],
	[-30, -40, -40, -50, -50, -40, -40, -30],
];

const MG_POS_BONUS_BLACK_KING: [[Value; 8]; 8] = [
	MG_POS_BONUS_WHITE_KING[7],
	MG_POS_BONUS_WHITE_KING[6],
	MG_POS_BONUS_WHITE_KING[5],
	MG_POS_BONUS_WHITE_KING[4],
	MG_POS_BONUS_WHITE_KING[3],
	MG_POS_BONUS_WHITE_KING[2],
	MG_POS_BONUS_WHITE_KING[1],
	MG_POS_BONUS_WHITE_KING[0],
];

/* Positional bonus. Use as: MG_POS_BONUS[color][piece][r][c]*/
const MG_POS_BONUS: [[[[Value; 8]; 8]; 6]; 2] = [
	[
		MG_POS_BONUS_WHITE_PAWN,
		MG_POS_BONUS_WHITE_KNIGHT,
		MG_POS_BONUS_WHITE_BISHOP,
		MG_POS_BONUS_WHITE_ROOK,
		MG_POS_BONUS_WHITE_QUEEN,
		MG_POS_BONUS_WHITE_KING,
	],
	[
		MG_POS_BONUS_BLACK_PAWN,
		MG_POS_BONUS_BLACK_KNIGHT,
		MG_POS_BONUS_BLACK_BISHOP,
		MG_POS_BONUS_BLACK_ROOK,
		MG_POS_BONUS_BLACK_QUEEN,
		MG_POS_BONUS_BLACK_KING,
	],
];

pub trait Valuable {
	fn value(&self) -> Value;
}

impl Valuable for Piece {
	#[inline]
	fn value(&self) -> Value {
		match &self {
			Piece::Pawn => 100,
			Piece::Knight => 320,
			Piece::Bishop => 330,
			Piece::Rook => 500,
			Piece::Queen => 900,
			Piece::King => 20000,
		}
	}
}

impl Valuable for Tile {
	#[inline]
	fn value(&self) -> Value {
		self.piece.value()
	}
}

impl Valuable for Board {
	fn value(&self) -> Value {
		let mut v = 0;
		for c in 0..8 {
			for r in 0..8 {
				let pos = Pos::at(c, r).unwrap();
				if let Some(tile) = self.at(pos) {
					match tile.color {
						Color::Black => {
							v -= tile.value();
							v -= MG_POS_BONUS[tile.color as usize][tile.piece as usize][r as usize][c as usize];
						}
						Color::White => {
							v += tile.value();
							v += MG_POS_BONUS[tile.color as usize][tile.piece as usize][r as usize][c as usize];
						}
					}
				}
			}
		}
		return v;
	}
}
