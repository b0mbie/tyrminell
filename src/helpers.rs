use crate::{
	csi::Csi,
	sgr::{
		Sgr, SgrColor
	}
};

use core::{
	fmt::{
		Display,
		Formatter,
		Result as FmtResult,
		Write,
	},
	num::NonZeroU8
};

#[inline(always)]
fn move_cursor_delta(
	w: &mut impl Write,
	neg: char, pos: char,
	delta: i8
) -> FmtResult {
	Csi::write_begin(w)?;
	write!(w, "{}", delta.abs())?;
	w.write_char(if delta < 0 { neg } else { pos })
}

/// Mode of erasing characters on the display.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraseDisplay {
	CurToEnd,
	CurToBegin,
	All,
	XtermAllNoScrollback,
}

impl Display for EraseDisplay {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Csi::write_begin(f)?;
		f.write_str(match self {
			EraseDisplay::CurToEnd => "0",
			EraseDisplay::CurToBegin => "1",
			EraseDisplay::All => "2",
			EraseDisplay::XtermAllNoScrollback => "3",
		})?;
		f.write_str("J")
	}
}

/// Mode of erasing characters in a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EraseLine {
	CurToEnd,
	CurToBegin,
	All,
}

impl Display for EraseLine {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Csi::write_begin(f)?;
		f.write_str(match self {
			EraseLine::CurToEnd => "0",
			EraseLine::CurToBegin => "1",
			EraseLine::All => "2",
		})?;
		f.write_str("K")
	}
}

/// Font weight change.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weight {
	Bold,
	Thin,
	#[default]
	Regular,
}

impl Weight {
	/// Convert `self` into an [`Sgr`].
	#[inline(always)]
	pub const fn into_sgr(self) -> Sgr {
		match self {
			Self::Bold => Sgr::WeightBoldOn,
			Self::Thin => Sgr::WeightThinOn,
			Self::Regular => Sgr::WeightAllOff,
		}
	}
}

impl From<Weight> for Sgr {
	#[inline(always)]
	fn from(val: Weight) -> Self {
		val.into_sgr()
	}
}

impl Display for Weight {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		self.into_sgr().fmt(f)
	}
}

/// Underline effect state change.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Underline {
	Single,
	Double,
	#[default]
	None,
}

impl Underline {
	/// Convert `self` into an [`Sgr`].
	#[inline(always)]
	pub const fn into_sgr(self) -> Sgr {
		match self {
			Self::None => Sgr::UnderlineNone,
			Self::Single => Sgr::UnderlineSingle,
			Self::Double => Sgr::UnderlineDouble,
		}
	}
}

impl From<Underline> for Sgr {
	#[inline(always)]
	fn from(val: Underline) -> Self {
		val.into_sgr()
	}
}

impl Display for Underline {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		self.into_sgr().fmt(f)
	}
}

/// Foreground or background color change.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
	#[default]
	Reset,
	Table(u8),
	Rgb(u8, u8, u8),
}

impl Color {
	/// Return an [`Sgr`] to change the foreground color to the one specified by
	/// `self`.
	#[inline(always)]
	pub const fn into_foreground(self) -> Sgr {
		match self {
			Self::Reset => Sgr::ForegroundDefault,
			Self::Table(n) => Sgr::Foreground(SgrColor::Table(n)),
			Self::Rgb(r, g, b) => Sgr::Foreground(SgrColor::Rgb(r, g, b)),
		}
	}

	/// Return an [`Sgr`] to change the background color to the one specified by
	/// `self`.
	#[inline(always)]
	pub const fn into_background(self) -> Sgr {
		match self {
			Self::Reset => Sgr::BackgroundDefault,
			Self::Table(n) => Sgr::Background(SgrColor::Table(n)),
			Self::Rgb(r, g, b) => Sgr::Background(SgrColor::Rgb(r, g, b)),
		}
	}
}

impl From<SgrColor> for Color {
	#[inline(always)]
	fn from(value: SgrColor) -> Self {
		match value {
			SgrColor::Table(n) => Self::Table(n),
			SgrColor::Rgb(r, g, b) => Self::Rgb(r, g, b),
		}
	}
}

/// Relative or absolute movement of the cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Movement {
	Relative {
		rows: Option<i8>,
		columns: Option<i8>,
	},
	Absolute {
		row: NonZeroU8,
		column: NonZeroU8,
	},
}

impl Display for Movement {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		match self {
			Movement::Relative { rows, columns } => {
				if let Some(x) = rows { move_cursor_delta(f, 'A', 'B', *x)?; }
				if let Some(x) = columns { move_cursor_delta(f, 'D', 'C', *x)?; }
				Ok(())
			}
			Movement::Absolute { row, column } => {
				Csi::write_begin(f)?;
				write!(f, "{row};{column}")?;
				f.write_str("J")
			}
		}
	}
}

/// Italic effect state change.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Italic {
	#[default]
	Off,
	On,
}

impl Italic {
	/// Convert `self` into an [`Sgr`].
	#[inline(always)]
	pub const fn into_sgr(self) -> Sgr {
		match self {
			Self::On => Sgr::ItalicOn,
			Self::Off => Sgr::ItalicOff,
		}
	}
}

impl From<Italic> for Sgr {
	#[inline(always)]
	fn from(val: Italic) -> Self {
		val.into_sgr()
	}
}

impl Display for Italic {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		self.into_sgr().fmt(f)
	}
}

/// Strikethrough effect state change.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Strikethrough {
	#[default]
	Off,
	On,
}

impl Strikethrough {
	/// Convert `self` into an [`Sgr`].
	#[inline(always)]
	pub const fn into_sgr(self) -> Sgr {
		match self {
			Self::On => Sgr::StrikethroughOn,
			Self::Off => Sgr::StrikethroughOff,
		}
	}
}

impl From<Strikethrough> for Sgr {
	#[inline(always)]
	fn from(val: Strikethrough) -> Self {
		val.into_sgr()
	}
}

impl Display for Strikethrough {
	#[inline(always)]
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		self.into_sgr().fmt(f)
	}
}

/// Graphics setting state change.
/// 
/// This structure combines multiple state changes into one escape sequence.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateChange {
	pub weight: Option<Weight>,
	pub italic: Option<Italic>,
	pub underline: Option<Underline>,
	pub strikethrough: Option<Strikethrough>,
	pub foreground: Option<Color>,
	pub background: Option<Color>,
}

const fn opt_replace_some<T: Copy>(option: Option<T>, value: T) -> Option<T> {
	match option {
		Some(..) => Some(value),
		None => None,
	}
}

impl StateChange {
	/// Create a [`StateChange`] that doesn't change any formatting.
	#[inline(always)]
	pub const fn new() -> Self {
		Self {
			weight: None,
			italic: None,
			underline: None,
			strikethrough: None,
			foreground: None,
			background: None,
		}
	}

	/// Create a [`StateChange`] that resets the effects of this one.
	#[inline(always)]
	pub const fn resetter(&self) -> Self {
		Self {
			weight: opt_replace_some(self.weight, Weight::Regular),
			italic: opt_replace_some(self.italic, Italic::Off),
			underline: opt_replace_some(self.underline, Underline::None),
			strikethrough: opt_replace_some(
				self.strikethrough, Strikethrough::Off
			),
			foreground: opt_replace_some(self.foreground, Color::Reset),
			background: opt_replace_some(self.background, Color::Reset),
		}
	}

	/// Set [`Self::weight`].
	#[inline(always)]
	pub const fn with_weight(self, x: Weight) -> Self {
		Self {
			weight: Some(x),
			..self
		}
	}

	/// Set [`Self::italic`].
	#[inline(always)]
	pub const fn with_italic(self, x: Italic) -> Self {
		Self {
			italic: Some(x),
			..self
		}
	}

	/// Set [`Self::underline`].
	#[inline(always)]
	pub const fn with_underline(self, x: Underline) -> Self {
		Self {
			underline: Some(x),
			..self
		}
	}

	/// Set [`Self::strikethrough`].
	#[inline(always)]
	pub const fn with_strikethrough(self, x: Strikethrough) -> Self {
		Self {
			strikethrough: Some(x),
			..self
		}
	}

	/// Set [`Self::foreground`].
	#[inline(always)]
	pub const fn with_foreground(self, x: Color) -> Self {
		Self {
			foreground: Some(x),
			..self
		}
	}

	/// Set [`Self::background`].
	#[inline(always)]
	pub const fn with_background(self, x: Color) -> Self {
		Self {
			background: Some(x),
			..self
		}
	}
}

impl Display for StateChange {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		Csi::write_begin(f)?;

		if let Some(weight) = self.weight {
			// Somehow, terminals can have characters that are bold _and_ thin
			// at the same time...?
			match weight {
				Weight::Bold => {
					Sgr::WeightAllOff.write_params_to(f)?;
					f.write_str(";")?;
					Sgr::WeightBoldOn.write_params_to(f)
				}
				Weight::Thin => {
					Sgr::WeightAllOff.write_params_to(f)?;
					f.write_str(";")?;
					Sgr::WeightThinOn.write_params_to(f)
				}
				Weight::Regular => Sgr::WeightAllOff.write_params_to(f),
			}?;
		}

		let mut is_first = true;

		if let Some(state) = self.italic {
			if is_first { f.write_str(";")?; is_first = false; }
			state.into_sgr().write_params_to(f)?;
		}

		if let Some(underline) = self.underline {
			if is_first { f.write_str(";")?; is_first = false; }
			match underline {
				Underline::None => Sgr::UnderlineNone,
				Underline::Single => Sgr::UnderlineSingle,
				Underline::Double => Sgr::UnderlineDouble,
			}.write_params_to(f)?;
		}

		if let Some(state) = self.strikethrough {
			if is_first { f.write_str(";")?; is_first = false; }
			state.into_sgr().write_params_to(f)?;
		}

		if let Some(color) = self.foreground {
			if is_first { f.write_str(";")?; is_first = false; }
			match color {
				Color::Reset => Sgr::ForegroundDefault,
				Color::Table(n) => Sgr::Foreground(SgrColor::Table(n)),
				Color::Rgb(r, g, b) => Sgr::Foreground(SgrColor::Rgb(r, g, b)),
			}.write_params_to(f)?;
		}
		if let Some(color) = self.background {
			if is_first { f.write_str(";")?; }
			match color {
				Color::Reset => Sgr::BackgroundDefault,
				Color::Table(n) => Sgr::Background(SgrColor::Table(n)),
				Color::Rgb(r, g, b) => Sgr::Background(SgrColor::Rgb(r, g, b)),
			}.write_params_to(f)?;
		}

		f.write_str(Csi::FINAL_STR)
	}
}
