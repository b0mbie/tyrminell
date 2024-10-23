use tyrminell::{
	StateChange, Color, Weight,
};

fn main() {
	println!(
		"{}Hello, {}world{}!",
		StateChange::new()
			.with_foreground(Color::Table(4)).with_weight(Weight::Bold),
		StateChange::new().with_foreground(Color::Table(2)),
		StateChange::new()
			.with_foreground(Color::Reset).with_weight(Weight::Regular)
	);
}
