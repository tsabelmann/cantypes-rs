use crate::constants::{*};


#[derive(Debug, PartialEq, Clone)]
pub struct CanFilter {
    is_extended: bool,
    can_id: u32,
    mask: u32
}

impl CanFilter {
    pub fn from_can_id(is_extended_id: bool, can_id: u32) -> Self {
        match is_extended_id {
            false => {
                Self {
                    is_extended: false,
                    can_id: can_id,
                    mask: STANDARD_FRAME_ID_MASK
                }
            },
            true => {
                Self {
                    is_extended: true,
                    can_id: can_id,
                    mask: EXTENDED_FRAME_ID_MASK
                }
            }
        }
    }


    pub fn match_can_id(&self, can_id: u32) -> bool {
        (self.can_id & self.mask) == (can_id & self.mask)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_filter_001() {
        assert!(CanFilter::from_can_id(false, 0xABC).match_can_id(0xABC));
    }
}