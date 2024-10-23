use core::iter::repeat;
use tyrminell::*;

fn main() {
	print!("{}", EraseDisplay::All);

	macro_rules! plainln {
		($($arg:tt)*) => {{
			print!("{}", Color::Reset.into_foreground());
			println!($($arg)*)
		}};
	}

	plainln!("0..16");
	for n in 0..16 {
		print!("{}█", Color::Table(n).into_foreground());
	}
	plainln!();

	for range in
		repeat(0..36u8)
			.take(6).enumerate()
			.map(move |(which, mut range)| {
				let offset = 16 + (which as u8) * 36u8;
				range.start += offset;
				range.end += offset;
				range
			})
	{
		plainln!("{}..{}", range.start, range.end);
		for n in range {
			print!("{}█", Color::Table(n).into_foreground());
		}
		plainln!();
	}

	plainln!("232..={}", u8::MAX);
	for n in 232..=u8::MAX {
		print!("{}█", Color::Table(n).into_foreground());
	}
	plainln!();

	print!("{}", Sgr::Reset);
}
