// SPDX-License-Identifier: MPL-2.0

use crate::csi::Csi;

use core::fmt;

/// Select Graphic Rendition, or **SGR**, CSI sequence parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sgr {
	Reset,
	#[doc(alias = "Bold")]
	WeightBoldOn,
	#[doc(alias = "Faint")]
	#[doc(alias = "Dim")]
	WeightThinOn,
	ItalicOn,
	UnderlineSingle,
	BlinkSlow,
	BlinkRapid,
	Invert,
	ConcealOn,
	StrikethroughOn,
	FontPrimary,
	Font1, Font2, Font3,
	Font4, Font5, Font6,
	Font7, Font8, Font9,
	#[doc(alias = "Gothic")]
	Fraktur,
	UnderlineDouble,
	WeightAllOff,
	ItalicOff,
	UnderlineNone,
	BlinkNone,
	ProportionalSpacingOn,
	ReversedOff,
	#[doc(alias = "Reveal")]
	ConcealOff,
	StrikethroughOff,
	Foreground1, Foreground2, Foreground3, Foreground4,
	Foreground5, Foreground6, Foreground7, Foreground8,
	Foreground(SgrColor),
	ForegroundDefault,
	Background1, Background2, Background3, Background4,
	Background5, Background6, Background7, Background8,
	Background(SgrColor),
	BackgroundDefault,
	ProportionalSpacingOff,
	#[doc(alias = "Framed")]
	FrameFramed,
	#[doc(alias = "Encircled")]
	FrameEncircled,
	OverlinedOn,
	FrameNone,
	OverlinedOff,
	UnderlineColor(SgrColor),
	UnderlineColorDefault,
}

impl Sgr {
	pub fn write_params_to(&self, w: &mut impl fmt::Write) -> fmt::Result {
		match self {
			Self::Reset => w.write_str("0"),
			Self::WeightBoldOn => w.write_str("1"),
			Self::WeightThinOn => w.write_str("2"),
			Self::ItalicOn => w.write_str("3"),
			Self::UnderlineSingle => w.write_str("4"),
			Self::BlinkSlow => w.write_str("5"),
			Self::BlinkRapid => w.write_str("6"),
			Self::Invert => w.write_str("7"),
			Self::ConcealOn => w.write_str("8"),
			Self::StrikethroughOn => w.write_str("9"),
			Self::FontPrimary => w.write_str("10"),
			Self::Font1 => w.write_str("11"),
			Self::Font2 => w.write_str("12"),
			Self::Font3 => w.write_str("13"),
			Self::Font4 => w.write_str("14"),
			Self::Font5 => w.write_str("15"),
			Self::Font6 => w.write_str("16"),
			Self::Font7 => w.write_str("17"),
			Self::Font8 => w.write_str("18"),
			Self::Font9 => w.write_str("19"),
			Self::Fraktur => w.write_str("20"),
			Self::UnderlineDouble => w.write_str("21"),
			Self::WeightAllOff => w.write_str("22"),
			Self::ItalicOff => w.write_str("23"),
			Self::UnderlineNone => w.write_str("24"),
			Self::BlinkNone => w.write_str("25"),
			Self::ProportionalSpacingOn => w.write_str("26"),
			Self::ReversedOff => w.write_str("27"),
			Self::ConcealOff => w.write_str("28"),
			Self::StrikethroughOff => w.write_str("29"),
			Self::Foreground1 => w.write_str("30"),
			Self::Foreground2 => w.write_str("31"),
			Self::Foreground3 => w.write_str("32"),
			Self::Foreground4 => w.write_str("33"),
			Self::Foreground5 => w.write_str("34"),
			Self::Foreground6 => w.write_str("35"),
			Self::Foreground7 => w.write_str("36"),
			Self::Foreground8 => w.write_str("37"),
			Self::Foreground(color) => {
				w.write_str("38;")?;
				write!(w, "{color}")
			}
			Self::ForegroundDefault => w.write_str("39"),
			Self::Background1 => w.write_str("40"),
			Self::Background2 => w.write_str("41"),
			Self::Background3 => w.write_str("42"),
			Self::Background4 => w.write_str("43"),
			Self::Background5 => w.write_str("44"),
			Self::Background6 => w.write_str("45"),
			Self::Background7 => w.write_str("46"),
			Self::Background8 => w.write_str("47"),
			Self::Background(color) => {
				w.write_str("48;")?;
				write!(w, "{color}")
			}
			Self::BackgroundDefault => w.write_str("49"),
			Self::ProportionalSpacingOff => w.write_str("50"),
			Self::FrameFramed => w.write_str("51"),
			Self::FrameEncircled => w.write_str("52"),
			Self::OverlinedOn => w.write_str("53"),
			Self::FrameNone => w.write_str("54"),
			Self::OverlinedOff => w.write_str("55"),
			Self::UnderlineColor(color) => {
				w.write_str("58;")?;
				write!(w, "{color}")
			}
			Self::UnderlineColorDefault => w.write_str("59"),
		}
	}
}

impl fmt::Display for Sgr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		Csi::write_begin(f)?;
		self.write_params_to(f)?;
		f.write_str(Csi::FINAL_STR)
	}
}

/// [`Sgr::Foreground`] and [`Sgr::Background`] parameters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SgrColor {
	Table(u8),
	Rgb(u8, u8, u8),
}

impl fmt::Display for SgrColor {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Table(n) => write!(f, "5;{n}"),
			Self::Rgb(r, g, b) => write!(f, "2;{r};{g};{b}"),
		}
	}
}
