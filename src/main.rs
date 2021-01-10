mod board;
mod generation;
mod minmax;

use crate::board::{Board, Color, Valuable};
use crate::minmax::{maximize, minimize};

fn main() {
	let mut b: Board = Board::new(Color::White);

	println!("The Board");
	for r in b.tiles.iter() {
		println!("{:?}", r);
	}
	println!("Initial board value = {}", &b.value());
	println!("");

	/* Just for testing: AI playing against itself in infinite loop */
	loop {
		let (score, mv) = maximize(&b, i32::MIN, i32::MAX, 0);
		let (f_pos, t_pos) = mv.unwrap();
		println!("{:?}: [{}{}]({})", b.player, f_pos, t_pos, score);
		b = b.clone_apply_move(&f_pos, &t_pos);

		let (score, mv) = minimize(&b, i32::MIN, i32::MAX, 0);
		let (f_pos, t_pos) = mv.unwrap();
		println!("{:?}: [{}{}]({})", b.player, f_pos, t_pos, score);
		b = b.clone_apply_move(&f_pos, &t_pos);
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
