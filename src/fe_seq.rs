use super::{
	byte_wrapper, byte_utf8able
};

use crate::csi::Csi;

/// ASCII escape character string.
pub const ESC_STR: &str = "\x1b";

/// `Fe`-type Escape sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeSeq<'a> {
	Pad,
	HighOctetPreset,
	BreakPermittedHere,
	NoBreakHere,
	Index,
	NextLine,
	StartOfSelArea,
	EndOfSelArea,
	HorizTabSet,
	RightJustify,
	VertTabSet,
	PartLineDown,
	PartLineUp,
	ReverseIndex,
	SingleShift2,
	SingleShift3,
	DeviceControlString(&'a [DcsChar]),
	PrivateUse1,
	PrivateUse2,
	SetTransmitState,
	CancelCharacter,
	MessageWaiting,
	StartOfProtArea,
	EndOfProtArea,
	StartOfString(&'a [SosChar]),
	/// Single Graphic Character Introducer.
	/// 
	/// Intended to allow an arbitrary Unicode character to be printed.
	/// This is to be followed by that character, most likely encoded in UTF-1.
	Sgci,
	/// Single Character Introducer.
	/// 
	/// To be followed by a [`Printable`] or [`FormatEffector`], which will be
	/// printed as ASCII no matter what graphic or control sets are in use.
	Sci,
	Csi(Csi<'a>),
	StringTerminator,
	OsCommand(&'a [Printable]),
	PrivacyMessage(&'a [Printable]),
	AppProgramCommand(&'a [Printable]),
}

impl<'a> FeSeq<'a> {
	/// Return the first byte after the escape character that represents this
	/// sequence.
	pub fn kind_byte(&self) -> u8 {
		match self {
			Self::Pad => 0x80,
			Self::HighOctetPreset => 0x81,
			Self::BreakPermittedHere => 0x82,
			Self::NoBreakHere => 0x83,
			Self::Index => 0x84,
			Self::NextLine => 0x85,
			Self::StartOfSelArea => 0x86,
			Self::EndOfSelArea => 0x87,
			Self::HorizTabSet => 0x88,
			Self::RightJustify => 0x89,
			Self::VertTabSet => 0x8a,
			Self::PartLineDown => 0x8b,
			Self::PartLineUp => 0x8c,
			Self::ReverseIndex => 0x8d,
			Self::SingleShift2 => 0x8e,
			Self::SingleShift3 => 0x8f,
			Self::DeviceControlString(..) => 0x90,
			Self::PrivateUse1 => 0x91,
			Self::PrivateUse2 => 0x92,
			Self::SetTransmitState => 0x93,
			Self::CancelCharacter => 0x94,
			Self::MessageWaiting => 0x95,
			Self::StartOfProtArea => 0x96,
			Self::EndOfProtArea => 0x97,
			Self::StartOfString(..) => 0x97,
			Self::Sgci => 0x99,
			Self::Sci => 0x9a,
			Self::Csi(..) => Csi::INTRO_BYTE,
			Self::StringTerminator => 0x9c,
			Self::OsCommand(..) => 0x9d,
			Self::PrivacyMessage(..) => 0x9e,
			Self::AppProgramCommand(..) => 0x9f,
		}
	}

	#[cfg(feature = "std")]
	/// Write this `Fe` Escape sequence to a destination.
	pub fn write_to(&self, w: &mut impl std::io::Write) -> std::io::Result<()> {
		w.write_all(ESC_STR.as_bytes())?;
		w.write_all(&[self.kind_byte()])?;
		match self {
			Self::DeviceControlString(chars) => {
				w.write_all(DcsChar::slice_as_bytes(chars))?;
				Self::StringTerminator.write_to(w)
			}
			Self::StartOfString(chars) => {
				w.write_all(SosChar::slice_as_bytes(chars))?;
				Self::StringTerminator.write_to(w)
			}
			Self::Csi(seq) => write!(w, "{seq}"),
			Self::OsCommand(chars) => {
				w.write_all(Printable::slice_as_bytes(chars))?;
				Self::StringTerminator.write_to(w)
			}
			Self::PrivacyMessage(chars) => {
				w.write_all(Printable::slice_as_bytes(chars))?;
				Self::StringTerminator.write_to(w)
			}
			Self::AppProgramCommand(chars) => {
				w.write_all(Printable::slice_as_bytes(chars))?;
				Self::StringTerminator.write_to(w)
			}
			_ => Ok(())
		}
	}
}

byte_wrapper! {
	for 0x08..=0x0d | 0x20..=0x7e =>
	#[repr(transparent)]
	/// Character type for [`FeSeq::DeviceControlString`].
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct DcsChar(u8);
}
byte_utf8able!(DcsChar);

byte_wrapper! {
	for 0x0..=0x96 | 0x98..=0x9b | 0x9d.. =>
	#[repr(transparent)]
	/// Character type for [`FeSeq::StartOfString`].
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct SosChar(u8);
}

impl From<Printable> for DcsChar {
	fn from(value: Printable) -> Self {
		unsafe { Self::new_unchecked(value.byte()) }
	}
}

impl From<FormatEffector> for DcsChar {
	fn from(value: FormatEffector) -> Self {
		unsafe { Self::new_unchecked(value.byte()) }
	}
}

byte_wrapper! {
	for 0x08..=0x0d =>
	#[repr(transparent)]
	/// Format effector character.
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct FormatEffector(u8);
}

byte_wrapper! {
	for 0x20..=0x7e =>
	#[repr(transparent)]
	/// ASCII printable character.
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct Printable(u8);
}
