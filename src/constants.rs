//! Module providing constants helpful in the context of CAN.
//!
//! The module contains the following constants:
//! - [STANDARD_FRAME_ID_MASK]
//! - [EXTENDED_FRAME_ID_MASK]
//! 

/// Constant defining the 11-bit mask for standard CAN-IDs.
pub const STANDARD_FRAME_ID_MASK: u32 = 0x00_00_07_FFu32;

/// Constant defining the 29-bit mask for extended CAN-IDs.
pub const EXTENDED_FRAME_ID_MASK: u32 = 0x1F_FF_FF_FFu32;
