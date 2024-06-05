use std::ops::Add;

use crate::constants::{*};


pub trait CanFilter {
    // fn _mask() -> u32;
    fn match_can_id(&self, can_id: u32) -> bool;
    fn can_id(&self) -> u32;
    fn mask(&self) -> u32; 
}

#[derive(PartialEq, Clone)]
struct StandardCanFilter {
    can_id: u32,
    mask: u32
}

impl StandardCanFilter {
    pub fn from_can_id(can_id: u32) -> Self {
        Self {
            can_id: can_id & STANDARD_FRAME_ID_MASK,
            mask: STANDARD_FRAME_ID_MASK
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
}

impl Default for StandardCanFilter {
    fn default() -> Self {
        Self {
            can_id: 0,
            mask: 0
        }
    }
}

impl Add<StandardCanFilter> for StandardCanFilter {
    type Output = StandardCanFilter;
    fn add(self, rhs: Self) -> Self::Output {
        let left_can_id_filtered = self.can_id & self.mask;
        let right_can_id_filtered = rhs.can_id & rhs.mask;
        let mask = (!left_can_id_filtered | right_can_id_filtered) & (!right_can_id_filtered | left_can_id_filtered);

        Self {
            can_id: self.can_id,
            mask: mask & STANDARD_FRAME_ID_MASK
        }
    }
}

impl Add<&StandardCanFilter> for StandardCanFilter {
    type Output = StandardCanFilter;
    fn add(self, rhs: &StandardCanFilter) -> Self::Output {
        self + rhs.clone()
    }
}

impl Add<&StandardCanFilter> for &StandardCanFilter {
    type Output = StandardCanFilter;
    fn add(self, rhs: &StandardCanFilter) -> Self::Output {
        self.clone() + rhs.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_filter_001() {
        assert!(StandardCanFilter::from_can_id(0xABC).match_can_id(0xABC));
    }

    #[test]
    fn match_filter_002() {
        assert!(!StandardCanFilter::from_can_id(0xABD).match_can_id(0xABC));
    }

    #[test]
    fn match_filter_003() {
        let filter = StandardCanFilter::from_can_id(0x7_FF) + StandardCanFilter::from_can_id(0x0_0F);
        println!("{:03X}", filter.mask());
        assert!(filter.mask() == 0x0_0F);
    }

    #[test]
    fn match_filter_004() {
        let filter = StandardCanFilter::default();
        for can_id in 0..STANDARD_FRAME_ID_MASK {
            assert!(filter.match_can_id(can_id));
        }        
    }
}