use crate::board::{Board, Color, Piece, Pos, Tile};

pub type Value = i16;

/* 
 * Simplified Evaluation Function: https://www.chessprogramming.org/Simplified_Evaluation_Function
 * Please note that the values presented here have been designed specifically to compensate for the lack of any other chess knowledge, and NOT for being supplemented by it.
*/

/* Value of each piece */
const MG: [Value; 6] = [ 100, 320, 330, 500, 900, 20000]; 

/* Pawn positional bonus */
const MG_POS_BONUS_PAWN: [[Value; 8]; 8] = [
	[  0,   0,   0,   0,   0,   0,   0,   0],
	[  5,  10,  10, -20, -20,  10,  10,   5],
	[  5,  -5, -10,   0,   0, -10,  -5,  -5],
	[  0,   0,   0,  20,  20,   0,   0,   0],
	[  5,   5,  10,  25,  25,  10,   5,   5],
	[ 10,  10,  20,  30,  30,  20,  10,  10],
	[ 50,  50,  50,  50,  50,  50,  50,  50],
	[  0,   0,   0,   0,   0,   0,   0,   0],
];

/* Knight positional bonus */
const MG_POS_BONUS_KNIGHT: [[Value; 8]; 8] = [
	[-50, -40, -30, -30, -30, -30, -40, -50],
	[-40, -20,   0,   0,   0,   0, -20, -40],
	[-30,   0,  10,  15,  15,  10,   0, -30],
	[-30,   5,  15,  20,  20,  15,   5, -30],
	[-30,   0,  15,  20,  20,  15,   0, -30],
	[-30,   5,  10,  15,  15,  10,   5, -30],
	[-40, -20,   0,   5,   5,   0, -20, -40],
	[-50, -40, -30, -30, -30, -30, -40, -50],
];

/* Bishop positional bonus */
const MG_POS_BONUS_BISHOP: [[Value; 8]; 8] = [
	[-20, -10, -10, -10, -10, -10, -10, -20],
	[-10,   5,   0,   0,   0,   0,   5, -10],
	[-10,  10,  10,  10,  10,  10,  10, -10],
	[-10,   0,  10,  10,  10,  10,   0, -10],
	[-10,   5,   5,  10,  10,   5,   5, -10],
	[-10,   0,   5,  10,  10,   5,   0, -10],
	[-10,   0,   0,   0,   0,   0,   0, -10],
	[-20, -10, -10, -10, -10, -10, -10, -20],
];

/* Rook positional bonus */
const MG_POS_BONUS_ROOK: [[Value; 8]; 8] = [
	[  0,   0,   0,   5,   5,   0,   0,   0],
	[ -5,   0,   0,   0,   0,   0,   0,  -5],
	[ -5,   0,   0,   0,   0,   0,   0,  -5],
	[ -5,   0,   0,   0,   0,   0,   0,  -5],
	[ -5,   0,   0,   0,   0,   0,   0,  -5],
	[ -5,   0,   0,   0,   0,   0,   0,  -5],
	[  5,  10,  10,  10,  10,  10,  10,   5],
	[  0,   0,   0,   0,   0,   0,   0,   0],
];

/* Queen positional bonus */
const MG_POS_BONUS_QUEEN: [[Value; 8]; 8] = [
	[-20, -10, -10,  -5,  -5, -10, -10, -20],
	[-10,   0,   5,   0,   0,   0,   0, -10],
	[-10,   5,   5,   5,   5,   5,   0, -10],
	[  0,   0,   5,   5,   5,   5,   0,  -5],
	[ -5,   0,   5,   5,   5,   5,   0,  -5],
	[-10,   0,   5,   5,   5,   5,   0, -10],
	[-10,   0,   0,   0,   0,   0,   0, -10],
	[-20, -10, -10,  -5,  -5, -10, -10, -20],
];

/* King positional bonus */
const MG_POS_BONUS_KING: [[Value; 8]; 8] = [
	[ 20,  30,  10,   0,   0,  10,  30,  20],
	[ 20,  20,   0,   0,   0,   0,  20,  20],
	[-10, -20, -20, -20, -20, -20, -20, -10],
	[-20, -30, -30, -40, -40, -30, -30, -20],
	[-30, -40, -40, -50, -50, -40, -40, -30],
	[-30, -40, -40, -50, -50, -40, -40, -30],
	[-30, -40, -40, -50, -50, -40, -40, -30],
	[-30, -40, -40, -50, -50, -40, -40, -30],
];

/* Positional bonus per piece */
const MG_POS_BONUS: [[[Value; 8]; 8]; 6] = [
	MG_POS_BONUS_PAWN,
	MG_POS_BONUS_KNIGHT,
	MG_POS_BONUS_BISHOP,
	MG_POS_BONUS_ROOK,
	MG_POS_BONUS_QUEEN,
	MG_POS_BONUS_KING,
];

pub trait Valuable {
	fn value(&self) -> Value;
}

impl Valuable for Piece {
	#[inline]
	fn value(&self) -> Value {
		MG[*self as usize]
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
		if let Some(v) = self.stored_value.get() {
			return v;
		}
		let mut v = 0;
		for c in 0..8 {
			for r in 0..8 {
				let pos = Pos::at(c, r).unwrap();
				if let Some(tile) = self.at(pos) {
					match tile.color {
						Color::Black => {
							v -= tile.value();
							v -= MG_POS_BONUS[tile.piece as usize][7-r as usize][7-c as usize];
						},
						Color::White => {
							v += tile.value();
							v += MG_POS_BONUS[tile.piece as usize][r as usize][c as usize];
						}
					}
				}
			}
		}
		self.stored_value.set(Some(v));
		v
	}
}
