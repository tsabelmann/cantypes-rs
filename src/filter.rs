//! Module providing filtering traits and filter implementations for standard and extended IDs.
//!

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

/// Private marker trait to inhibit the external implementations of the [CanIdFilter] trait.
trait CanIdFilterPrivateMarker {}

/// The trait defines the interface for CAN-filtering based on CAN-IDs and acceptance masks. 
#[allow(private_bounds)]
pub trait CanIdFilter: CanIdFilterPrivateMarker {
    /// Checks, whether the CAN-ID is accepted by the filter.
    fn match_can_id(&self, can_id: u32) -> bool;
    /// Retrieves the CAN-ID the filter is based on.
    fn can_id(&self) -> u32;
    /// Retrieves the set or computed acceptance mask.
    fn mask(&self) -> u32; 
    /// Retrieves if the filter is for standard or extended CAN-IDs.
    fn mask_type(&self) -> MaskType;
    /// Computes the weight, i.e., the number of matching/accepting CAN-IDs. 
    fn weight(&self) -> u32;
}

/// The struct for modelling standard filter.
#[derive(PartialEq, Clone)]
pub struct StandardCanIdFilter {
    /// The CAN-ID the filter is based on.
    can_id: u32,
    /// The set or computed acceptance mask.
    mask: u32
}

impl CanIdFilterPrivateMarker for StandardCanIdFilter {}

impl StandardCanIdFilter {
    /// Constructs a standard filter from a CAN-ID.
    /// 
    /// # Example
    /// ```
    /// use cantypes::filter::{StandardCanIdFilter, CanIdFilter};
    /// 
    /// let filter = StandardCanIdFilter::from_can_id(0xABC);
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
    /// use cantypes::filter::{StandardCanIdFilter, CanIdFilter};
    /// 
    /// let filter = StandardCanIdFilter::accept_all();
    /// assert!(filter.match_can_id(0xABC));
    /// ```
    pub fn accept_all() -> Self {
        Self {
            can_id: 0,
            mask: 0
        }
    }
}

impl CanIdFilter for StandardCanIdFilter {    
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

    fn weight(&self) -> u32 {
        let mut counter = 0;
        for i in 0..STANDARD_FRAME_ID_LENGTH {
            if self.mask & (1 << i) == 0 {
                counter += 1;
            }
        }
        1 << counter
    }
}

impl Add<StandardCanIdFilter> for StandardCanIdFilter {
    type Output = StandardCanIdFilter;
    /// Constructs a combined filter.
    /// 
    /// # Example
    /// 
    /// ```
    /// use cantypes::filter::{StandardCanIdFilter, CanIdFilter};
    /// 
    /// let f1 = StandardCanIdFilter::from_can_id(0x180102AE);
    /// let f2 = StandardCanIdFilter::from_can_id(0x180304AE);
    /// let f3 = f1 + f2;
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

impl Add<&StandardCanIdFilter> for StandardCanIdFilter {
    type Output = StandardCanIdFilter;
    /// Constructs a combined filter.
    /// 
    /// # Example
    /// 
    /// ```
    /// use cantypes::filter::{StandardCanIdFilter, CanIdFilter};
    /// 
    /// let f1 = StandardCanIdFilter::from_can_id(0x180102AE);
    /// let f2 = StandardCanIdFilter::from_can_id(0x180304AE);
    /// let f3 = f1 + &f2;
    /// ```
    fn add(self, rhs: &StandardCanIdFilter) -> Self::Output {
        &self + rhs
    }
}

impl Add<&StandardCanIdFilter> for &StandardCanIdFilter {
    type Output = StandardCanIdFilter;
    /// Constructs a combined filter.
    /// 
    /// # Example
    /// 
    /// ```
    /// use cantypes::filter::{StandardCanIdFilter, CanIdFilter};
    /// 
    /// let f1 = StandardCanIdFilter::from_can_id(0x180102AE);
    /// let f2 = StandardCanIdFilter::from_can_id(0x180304AE);
    /// let f3 = &f1 + &f2;
    /// ```
    fn add(self, rhs: &StandardCanIdFilter) -> Self::Output {
        let left_can_id_filtered = self.can_id() & self.mask();
        let right_can_id_filtered = rhs.can_id() & rhs.mask();
        let mask = (!left_can_id_filtered | right_can_id_filtered) & (!right_can_id_filtered | left_can_id_filtered);

        StandardCanIdFilter {
            can_id: self.can_id,
            mask: mask & STANDARD_FRAME_ID_MASK
        }
    }
}

/// The struct for modelling extended filter.
#[derive(PartialEq, Clone)]
pub struct ExtendedCanIdFilter {
    /// The CAN-ID the filter is based on.
    can_id: u32,
    /// The set or computed acceptance mask.
    mask: u32
}

impl ExtendedCanIdFilter {
    /// Constructs an extended filter from a CAN-ID.
    /// 
    /// # Example
    /// ```
    /// use cantypes::filter::{ExtendedCanIdFilter, CanIdFilter};
    /// 
    /// let filter = ExtendedCanIdFilter::from_can_id(0x18ABCDEF);
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
    /// use cantypes::filter::{ExtendedCanIdFilter, CanIdFilter};
    /// 
    /// let filter = ExtendedCanIdFilter::accept_all();
    /// assert!(filter.match_can_id(0x180102AE));
    /// ```
    pub fn accept_all() -> Self {
        Self {
            can_id: 0,
            mask: 0
        }
    }
}

impl CanIdFilterPrivateMarker for ExtendedCanIdFilter {}

impl CanIdFilter for ExtendedCanIdFilter {
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

    fn weight(&self) -> u32 {
        let mut counter = 0;
        for i in 0..EXTENDED_FRAME_ID_LENGTH {
            if self.mask & (1 << i) == 0 {
                counter += 1;
            }
        }
        1 << counter
    }
}

/// Implemented [std::ops::Add] trait to combine [ExtendedCanIdFilter] in a convenient way.
impl Add<ExtendedCanIdFilter> for ExtendedCanIdFilter {
    type Output = ExtendedCanIdFilter;
    /// Constructs a combined filter.
    /// 
    /// # Example
    /// 
    /// ```
    /// use cantypes::filter::{ExtendedCanIdFilter, CanIdFilter};
    /// 
    /// let f1 = ExtendedCanIdFilter::from_can_id(0x180102AE);
    /// let f2 = ExtendedCanIdFilter::from_can_id(0x180304AE);
    /// let f3 = f1 + f2;
    /// ```
    fn add(self, rhs: Self) -> Self::Output {
        &self + &rhs
    }
}

/// Implemented [std::ops::Add] trait to combine [ExtendedCanIdFilter] in a convenient way.
impl Add<&ExtendedCanIdFilter> for ExtendedCanIdFilter {
    type Output = ExtendedCanIdFilter;
    /// Constructs a combined filter.
    /// 
    /// # Example
    /// 
    /// ```
    /// use cantypes::filter::{ExtendedCanIdFilter, CanIdFilter};
    /// 
    /// let f1 = ExtendedCanIdFilter::from_can_id(0x180102AE);
    /// let f2 = ExtendedCanIdFilter::from_can_id(0x180304AE);
    /// let f3 = f1 + &f2;
    /// ```
    fn add(self, rhs: &ExtendedCanIdFilter) -> Self::Output {
        &self + rhs
    }
}

/// Implemented [std::ops::Add] trait to combine [ExtendedCanIdFilter] in a convenient way.
impl Add<&ExtendedCanIdFilter> for &ExtendedCanIdFilter {
    type Output = ExtendedCanIdFilter;
    /// Constructs a combined filter.
    /// 
    /// # Example
    /// 
    /// ```
    /// use cantypes::filter::{ExtendedCanIdFilter, CanIdFilter};
    /// 
    /// let f1 = ExtendedCanIdFilter::from_can_id(0x180102AE);
    /// let f2 = ExtendedCanIdFilter::from_can_id(0x180304AE);
    /// let f3 = &f1 + &f2;
    /// ```
    fn add(self, rhs: &ExtendedCanIdFilter) -> Self::Output {
        let left_can_id_filtered = self.can_id() & self.mask();
        let right_can_id_filtered = rhs.can_id() & rhs.mask();
        let mask = (!left_can_id_filtered | right_can_id_filtered) & (!right_can_id_filtered | left_can_id_filtered);

        ExtendedCanIdFilter {
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
        assert!(StandardCanIdFilter::from_can_id(0xABC).match_can_id(0xABC));
    }

    #[test]
    fn match_std_filter_002() {
        assert!(!StandardCanIdFilter::from_can_id(0xABD).match_can_id(0xABC));
    }

    #[test]
    fn match_std_filter_003() {
        let filter = StandardCanIdFilter::from_can_id(0x7_FF) + StandardCanIdFilter::from_can_id(0x0_0F);
        println!("{:03X}", filter.mask());
        assert!(filter.mask() == 0x0_0F);
    }

    #[test]
    fn match_std_filter_004() {
        let filter = StandardCanIdFilter::accept_all();
        for can_id in 0..STANDARD_FRAME_ID_MASK {
            assert!(filter.match_can_id(can_id));
        }        
    }

    #[test]
    fn weight_std_filter_001() {
        let filter = StandardCanIdFilter::accept_all();
        assert_eq!(filter.weight(), 0x800);
    }

    #[test]
    fn match_ext_filter_001() {
        assert!(ExtendedCanIdFilter::from_can_id(0xABC).match_can_id(0xABC));
    }

    #[test]
    fn match_ext_filter_002() {
        assert!(!ExtendedCanIdFilter::from_can_id(0xABD).match_can_id(0xABC));
    }

    #[test]
    fn match_ext_filter_003() {
        let filter = ExtendedCanIdFilter::from_can_id(0x1F_FF_CC_FF) + ExtendedCanIdFilter::from_can_id(0x1F_FF_33_FF);
        println!("{:03X}", filter.mask());
        assert!(filter.mask() == 0x1F_FF_00_FF);
    }

    #[test]
    fn match_ext_filter_004() {
        let filter = ExtendedCanIdFilter::accept_all();
        for can_id in 0..EXTENDED_FRAME_ID_MASK {
            assert!(filter.match_can_id(can_id));
        }        
    }

    #[test]
    fn weight_ext_filter_001() {
        let filter = ExtendedCanIdFilter::accept_all();
        assert_eq!(filter.weight(), 0x20_00_00_00);
    }
}