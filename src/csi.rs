use super::{
	byte_wrapper, byte_utf8able
};

use core::fmt;

/// Control Sequence Introducer, or **CSI**, sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Csi<'a> {
	/// CSI parameter bytes. See [`CsiParam`].
	pub parameter_bytes: &'a [CsiParam],
	/// CSI intermediate bytes. See [`CsiInter`].
	pub intermediate_bytes: &'a [CsiInter],
	/// CSI final byte. See [`CsiFinal`].
	pub final_byte: CsiFinal,
}

impl<'a> Csi<'a> {
	pub const INTRO_BYTE: u8 = b'[';
	pub const INTRO_CHAR: char = '[';
	pub const INTRO_STR: &'static str = "[";
	pub const FINAL_BYTE: CsiFinal = unsafe { CsiFinal::new_unchecked(b'm') };
	pub const FINAL_CHAR: char = 'm';
	pub const FINAL_STR: &'static str = "m";

	/// Write the beginning of a CSI sequence.
	pub fn write_begin(w: &mut impl fmt::Write) -> fmt::Result {
		w.write_str(crate::fe_seq::ESC_STR)?;
		w.write_str(Self::INTRO_STR)
	}
}

impl<'a> fmt::Display for Csi<'a> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.write_str(crate::fe_seq::ESC_STR)?;
		f.write_str(Self::INTRO_STR)?;
		f.write_str(CsiParam::slice_as_str(self.parameter_bytes))?;
		f.write_str(CsiInter::slice_as_str(self.intermediate_bytes))?;
		f.write_str(self.final_byte.as_str())
	}
}

byte_wrapper! {
	for 0x30..=0x3f =>
	#[repr(transparent)]
	/// [`Csi`] parameter byte.
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct CsiParam(u8);
}
byte_utf8able!(CsiParam);

byte_wrapper! {
	for 0x20..=0x2f =>
	#[repr(transparent)]
	/// [`Csi`] intermediate byte.
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct CsiInter(u8);
}
byte_utf8able!(CsiInter);

byte_wrapper! {
	for 0x40..=0x7e =>
	#[repr(transparent)]
	/// [`Csi`] final byte.
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub struct CsiFinal(u8);
}
byte_utf8able!(CsiFinal);
