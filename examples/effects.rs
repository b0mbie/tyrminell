use tyrminell::{
	Italic,
	Underline,
	Strikethrough,
	Sgr,
};

fn main() {
	print!("Regular");
	print!(" {}Italic{}", Italic::On, Italic::Off);
	print!(" {}Single Underline{}", Underline::Single, Underline::None);
	print!(" {}Double Underline{}", Underline::Double, Underline::None);
	print!(" {}Strikethrough{}", Strikethrough::On, Strikethrough::Off);
	println!("{}", Sgr::Reset);
}
