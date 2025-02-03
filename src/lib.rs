// SPDX-License-Identifier: MPL-2.0

//! Interact with the terminal to adjust printed text, with minimal heap
//! allocations.
//! 
//! Attempt to conform to
//! <https://www.ecma-international.org/publications-and-standards/standards/ecma-48/>.
//! 
//! Many items in this crate are [`Display`](core::fmt::Display)-able, which
//! means that you can use them with `print!` and `println!`.

#![cfg_attr(not(feature = "std"), no_std)]

mod csi;
pub use csi::*;
mod fe_seq;
pub use fe_seq::*;
mod sgr;
pub use sgr::*;

macro_rules! byte_wrapper {
	{
		for $ptn:pat =>
		#[repr(transparent)]
		$(#[$attr:meta])*
		pub struct $name:ident(u8);
	} => {
		$(#[$attr])*
		#[repr(transparent)]
		pub struct $name(u8);

		impl $name {
			/// Create an instance of this type from a byte, checking if the
			/// specified byte is valid.
			/// 
			/// If the byte is not valid, this function returns `None`.
			#[inline(always)]
			pub const fn new(byte: u8) -> Option<Self> {
				match byte {
					$ptn => Some(Self(byte)),
					_ => None,
				}
			}

			/// Create an instance of this type from a byte, without checking if
			/// it's valid.
			/// 
			/// See also [`Self::new`].
			/// 
			/// # Safety
			/// The specified byte must be valid for this wrapper. See [`Self`].
			#[inline(always)]
			pub const unsafe fn new_unchecked(byte: u8) -> Self {
				Self(byte)
			}

			/// Return `true` if `byte` is valid for this wrapper, or `false`
			/// otherwise.
			#[inline(always)]
			pub const fn is_byte_valid(byte: u8) -> bool {
				match byte {
					$ptn => true,
					_ => false,
				}
			}

			/// Returns the byte that is represented by this wrapper.
			#[inline(always)]
			pub const fn byte(&self) -> u8 {
				self.0
			}

			/// Convert a slice of [`Self`] to a slice of [`u8`].
			#[inline(always)]
			pub const fn slice_as_bytes(slice: &[Self]) -> &[u8] {
				// SAFETY: `Self` wraps a `u8` using `repr(transparent)`.
				unsafe {
					core::slice::from_raw_parts(
						slice.as_ptr() as *const _,
						slice.len()
					)
				}
			}

			/// Try to convert a slice of [`u8`] to a slice of [`Self`].
			/// 
			/// If one `u8` is not valid as per [`Self::is_byte_valid`], then
			/// this function will return [`Err`] with the position of the
			/// first invalid byte encountered.
			pub fn slice_from_bytes(bytes: &[u8]) -> Result<&[Self], usize> {
				if let Some(idx) = bytes.iter().copied().enumerate()
					.find_map(move |(idx, byte)| {
						(!Self::is_byte_valid(byte)).then_some(idx)
					})
				{
					Err(idx)
				} else {
					Ok(unsafe { Self::slice_from_bytes_unchecked(bytes) })
				}
			}

			/// Convert a slice of [`u8`] to a slice of [`Self`].
			/// 
			/// See also [`Self::slice_from_bytes`].
			/// 
			/// # Safety
			/// `bytes` must only contain bytes that are valid for
			/// [`Self::new_unchecked`].
			#[inline(always)]
			pub const unsafe fn slice_from_bytes_unchecked(
				bytes: &[u8]
			) -> &[Self] {
				// SAFETY: `Self` wraps a `u8` using `repr(transparent)`.
				unsafe {
					core::slice::from_raw_parts(
						bytes.as_ptr() as *const _,
						bytes.len()
					)
				}
			}
		}
	};
}
pub(crate) use byte_wrapper;

macro_rules! byte_utf8able {
	($name:ident) => {
		impl $name {
			/// Convert [`Self`] to a [`str`].
			/// 
			/// Since the inner byte is always a valid codepoint by itself, this
			/// is a safe operation.
			#[inline(always)]
			pub const fn as_str(&self) -> &str {
				// SAFETY: The inner byte is always valid UTF-8.
				unsafe { core::str::from_utf8_unchecked(
					core::slice::from_ref(&self.0)
				) }
			}
		
			/// Convert [`Self`] to a [`char`].
			/// 
			/// Since the inner byte is always a valid codepoint by itself, this
			/// is a safe operation.
			#[inline(always)]
			pub const fn as_char(&self) -> char {
				// SAFETY: The inner byte is always valid UTF-8.
				unsafe { char::from_u32_unchecked(self.0 as _) }
			}

			/// Convert a slice of [`Self`] to a [`str`].
			/// 
			/// Since the slice consists of bytes which are valid codepoints by
			/// themselves, this is a safe operation.
			#[inline(always)]
			pub const fn slice_as_str(slice: &[Self]) -> &str {
				// SAFETY: Every single inner byte is a valid codepoint by itself.
				unsafe { core::str::from_utf8_unchecked(
					Self::slice_as_bytes(slice)
				) }
			}
		}

		impl core::fmt::Display for $name {
			#[inline(always)]
			fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
				f.write_str(self.as_str())
			}
		}
	};
}
pub(crate) use byte_utf8able;

#[cfg(feature = "helpers")]
mod helpers;
#[cfg(feature = "helpers")]
pub use helpers::*;
