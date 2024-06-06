//! Module providing constants helpful in the context of CAN.
//!
//! The module contains the following constants:
//! - [STANDARD_FRAME_ID_MASK]
//! - [EXTENDED_FRAME_ID_MASK]
//! - [STANDARD_FRAME_ID_LENGTH]
//! - [EXTENDED_FRAME_ID_LENGTH]

/// Constant defining the 11-bit mask for standard CAN-IDs.
pub const STANDARD_FRAME_ID_MASK: u32 = 0x00_00_07_FFu32;

/// Constant defining the number of bits of a standard CAN-ID.
pub const STANDARD_FRAME_ID_LENGTH: u32 = 11u32;

/// Constant defining the 29-bit mask for extended CAN-IDs.
pub const EXTENDED_FRAME_ID_MASK: u32 = 0x1F_FF_FF_FFu32;

/// Constant defining the number of bits of a extended CAN-ID.
pub const EXTENDED_FRAME_ID_LENGTH: u32 = 29u32;