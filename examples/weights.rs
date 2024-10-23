use tyrminell::{
	Sgr,
	Weight
};

fn main() {
	print!("{}Regular", Weight::Regular);
	print!(" {}Bold", Weight::Bold);
	print!(" {}Thin", Weight::Thin);
	println!("{}", Sgr::Reset);
}
