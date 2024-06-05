//! Module providing filtering traits and filter implementations for standard and extended IDs.
//!
//! The [CANRead] trait provides methods and retrieve the [DLC](CANRead::dlc), the number of
//! available bytes, and a [data](CANRead::data) slice that is read-only. In theory, only the
//! [data](CANRead::data) slice is needed since one can retrieve the DLC from the slice as well.
//!
//! The [CANWrite] trait provides one additional methods. The [mut_data](CANWrite::mut_data) method
//! allows for mutating the slice.


use std::ops::Add;
use crate::constants::{*};

/// A marker type the signals if the filter is for standard or extended filtering.
#[derive(Debug, PartialEq, Clone)]
pub enum MaskType {
    /// The CAN filter is designated to filter standard frames.
    Standard,
    /// The CAN filter is designated to filter extended frames.
    Extended
}

/// Private marker trait to inhibit the external implementations of the [CanFilter] trait.
trait CanFilterPrivateMarker {}

/// The trait defines the interface for CAN-filtering based on CAN-IDs and acceptance masks. 
#[allow(private_bounds)]
pub trait CanFilter: CanFilterPrivateMarker {
    /// Checks, whether the [can_id] is accepted by the filter.
    fn match_can_id(&self, can_id: u32) -> bool;
    /// Retrieves the CAN-ID the filter is based on.
    fn can_id(&self) -> u32;
    /// Retrieves the set or computed acceptance mask.
    fn mask(&self) -> u32; 
    /// Retrieves if the filter is for standard or extended CAN-IDs.
    fn mask_type(&self) -> MaskType;
}

/// The struct for modelling standard filter.
#[derive(PartialEq, Clone)]
pub struct StandardCanFilter {
    /// The CAN-ID the filter is based on.
    can_id: u32,
    /// The set or computed acceptance mask.
    mask: u32
}

impl CanFilterPrivateMarker for StandardCanFilter {}

impl StandardCanFilter {
    /// Constructs a standard filter from a CAN-ID.
    /// 
    /// # Example
    /// ```
    /// use cantypes::filter::{StandardCanFilter, CanFilter};
    /// 
    /// let filter = StandardCanFilter::from_can_id(0xABC);
    /// assert!(filter.match_can_id(0xABC));
    /// ```
    pub fn from_can_id(can_id: u32) -> Self {
        Self {
            can_id: can_id & STANDARD_FRAME_ID_MASK,
            mask: STANDARD_FRAME_ID_MASK
        }
    }

    /// Constructs a standard filter that accepts any CAN-ID.
    /// 
    /// # Example
    /// ```
    /// use cantypes::filter::{StandardCanFilter, CanFilter};
    /// 
    /// let filter = StandardCanFilter::accept_all();
    /// assert!(filter.match_can_id(0xABC));
    /// ```
    pub fn accept_all() -> Self {
        Self {
            can_id: 0,
            mask: 0
        }
    }
}

impl CanFilter for StandardCanFilter {    
    fn match_can_id(&self, can_id: u32) -> bool {
        (self.can_id & self.mask) == (can_id & self.mask)
    }

    fn can_id(&self) -> u32 {
        self.can_id
    }

    fn mask(&self) -> u32 {
        self.mask
    }

    fn mask_type(&self) -> MaskType {
        MaskType::Standard
    }
}

impl Add<StandardCanFilter> for StandardCanFilter {
    type Output = StandardCanFilter;
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add<&StandardCanFilter> for StandardCanFilter {
    type Output = StandardCanFilter;
    fn add(self, rhs: &StandardCanFilter) -> Self::Output {
        &self + rhs
    }
}

impl Add<&StandardCanFilter> for &StandardCanFilter {
    type Output = StandardCanFilter;
    fn add(self, rhs: &StandardCanFilter) -> Self::Output {
        let left_can_id_filtered = self.can_id() & self.mask();
        let right_can_id_filtered = rhs.can_id() & rhs.mask();
        let mask = (!left_can_id_filtered | right_can_id_filtered) & (!right_can_id_filtered | left_can_id_filtered);

        StandardCanFilter {
            can_id: self.can_id,
            mask: mask & STANDARD_FRAME_ID_MASK
        }
    }
}

/// The struct for modelling extended filter.
#[derive(PartialEq, Clone)]
pub struct ExtendedCanFilter {
    /// The CAN-ID the filter is based on.
    can_id: u32,
    /// The set or computed acceptance mask.
    mask: u32
}

impl ExtendedCanFilter {
    /// Constructs an extended filter from a CAN-ID.
    /// 
    /// # Example
    /// ```
    /// use cantypes::filter::{ExtendedCanFilter, CanFilter};
    /// 
    /// let filter = ExtendedCanFilter::from_can_id(0x18ABCDEF);
    /// assert!(filter.match_can_id(0x18ABCDEF));
    /// ```
    pub fn from_can_id(can_id: u32) -> Self {
        Self {
            can_id: can_id & EXTENDED_FRAME_ID_MASK,
            mask: EXTENDED_FRAME_ID_MASK
        }
    }

    /// Constructs an extended filter that accepts any CAN-ID.
    /// 
    /// # Example
    /// ```
    /// use cantypes::filter::{ExtendedCanFilter, CanFilter};
    /// 
    /// let filter = ExtendedCanFilter::accept_all();
    /// assert!(filter.match_can_id(0x180102AE));
    /// ```
    pub fn accept_all() -> Self {
        Self {
            can_id: 0,
            mask: 0
        }
    }
}

impl CanFilterPrivateMarker for ExtendedCanFilter {}

impl CanFilter for ExtendedCanFilter {
    fn match_can_id(&self, can_id: u32) -> bool {
        (self.can_id & self.mask) == (can_id & self.mask)
    }

    fn can_id(&self) -> u32 {
        self.can_id
    }

    fn mask(&self) -> u32 {
        self.mask
    }

    fn mask_type(&self) -> MaskType {
        MaskType::Extended
    }
}

/// Implemented [std::ops::Add] trait to combine [ExtendedCanFilter] in a convenient way.
impl Add<ExtendedCanFilter> for ExtendedCanFilter {
    type Output = ExtendedCanFilter;
    /// Constructs a combined filter.
    /// 
    /// # Example
    /// 
    /// ```
    /// use cantypes::filter::{ExtendedCanFilter, CanFilter};
    /// 
    /// let f1 = ExtendedCanFilter::from_can_id(0x180102AE);
    /// let f2 = ExtendedCanFilter::from_can_id(0x180304AE);
    /// let f3 = f1 + f2;
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

/// Implemented [std::ops::Add] trait to combine [ExtendedCanFilter] in a convenient way.
impl Add<&ExtendedCanFilter> for ExtendedCanFilter {
    type Output = ExtendedCanFilter;
    /// Constructs a combined filter.
    /// 
    /// # Example
    /// 
    /// ```
    /// use cantypes::filter::{ExtendedCanFilter, CanFilter};
    /// 
    /// let f1 = ExtendedCanFilter::from_can_id(0x180102AE);
    /// let f2 = ExtendedCanFilter::from_can_id(0x180304AE);
    /// let f3 = f1 + &f2;
    /// ```
    fn add(self, rhs: &ExtendedCanFilter) -> Self::Output {
        &self + rhs
    }
}

/// Implemented [std::ops::Add] trait to combine [ExtendedCanFilter] in a convenient way.
impl Add<&ExtendedCanFilter> for &ExtendedCanFilter {
    type Output = ExtendedCanFilter;
    /// Constructs a combined filter.
    /// 
    /// # Example
    /// 
    /// ```
    /// use cantypes::filter::{ExtendedCanFilter, CanFilter};
    /// 
    /// let f1 = ExtendedCanFilter::from_can_id(0x180102AE);
    /// let f2 = ExtendedCanFilter::from_can_id(0x180304AE);
    /// let f3 = &f1 + &f2;
    /// ```
    fn add(self, rhs: &ExtendedCanFilter) -> Self::Output {
        let left_can_id_filtered = self.can_id() & self.mask();
        let right_can_id_filtered = rhs.can_id() & rhs.mask();
        let mask = (!left_can_id_filtered | right_can_id_filtered) & (!right_can_id_filtered | left_can_id_filtered);

        ExtendedCanFilter {
            can_id: self.can_id,
            mask: mask & EXTENDED_FRAME_ID_MASK
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_std_filter_001() {
        assert!(StandardCanFilter::from_can_id(0xABC).match_can_id(0xABC));
    }

    #[test]
    fn match_std_filter_002() {
        assert!(!StandardCanFilter::from_can_id(0xABD).match_can_id(0xABC));
    }

    #[test]
    fn match_std_filter_003() {
        let filter = StandardCanFilter::from_can_id(0x7_FF) + StandardCanFilter::from_can_id(0x0_0F);
        println!("{:03X}", filter.mask());
        assert!(filter.mask() == 0x0_0F);
    }

    #[test]
    fn match_std_filter_004() {
        let filter = StandardCanFilter::accept_all();
        for can_id in 0..STANDARD_FRAME_ID_MASK {
            assert!(filter.match_can_id(can_id));
        }        
    }


    #[test]
    fn match_ext_filter_001() {
        assert!(ExtendedCanFilter::from_can_id(0xABC).match_can_id(0xABC));
    }

    #[test]
    fn match_ext_filter_002() {
        assert!(!ExtendedCanFilter::from_can_id(0xABD).match_can_id(0xABC));
    }

    #[test]
    fn match_ext_filter_003() {
        let filter = ExtendedCanFilter::from_can_id(0x1F_FF_CC_FF) + ExtendedCanFilter::from_can_id(0x1F_FF_33_FF);
        println!("{:03X}", filter.mask());
        assert!(filter.mask() == 0x1F_FF_00_FF);
    }

    #[test]
    fn match_ext_filter_004() {
        let filter = ExtendedCanFilter::accept_all();
        for can_id in 0..EXTENDED_FRAME_ID_MASK {
            assert!(filter.match_can_id(can_id));
        }        
    }

}