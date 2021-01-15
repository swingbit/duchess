mod board;
mod generation;
mod evaluation;
mod minmax;
mod uci;

use crate::board::{Board, Color};
use crate::minmax::{minmax};
use crate::uci::{ucitest};

extern crate vampirc_uci;

fn self_play_test() {
	let mut b: Board = Board::new(Color::White);

	/* Just for testing: AI playing against itself in infinite loop */
	loop {
		let (score, mv) = minmax(&b);
		println!("{:?}: [{}{}]({})", b.player, mv.f_pos, mv.t_pos, score);
		b = b.clone_apply_move(mv.f_pos, mv.t_pos);
	}
}
fn main() {
	// self_play_test();
	ucitest();
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
