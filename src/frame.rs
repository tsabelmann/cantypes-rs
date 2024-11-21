//! Module providing CAN frame definitions and traits to easily create, manipulate CAN frames and access their data.

use std::fmt::Debug;
use crate::{EXTENDED_FRAME_ID_MASK, STANDARD_FRAME_ID_MASK};


#[derive(Clone, PartialEq, Debug)]
pub enum IdType {
    /// The CAN-ID is of type standard. Thus, the CAN-ID consists of 11-bits. See also: [`STANDARD_FRAME_ID_LENGTH`](crate::constants::STANDARD_FRAME_ID_LENGTH).
    Standard,
    /// The CAN-ID is of type extended. Thus, the CAN-ID consists of 29-bits. See also: [`EXTENDED_FRAME_ID_LENGTH`](crate::constants::EXTENDED_FRAME_ID_LENGTH).
    Extended
}

#[derive(Clone, PartialEq)]
pub enum CanFrame {
    DataFrame {
        can_id: u32,
        id_type: IdType,
        dlc: u8,
        data: [u8; 8]
    },
    RemoteFrame {
        can_id: u32,
        id_type: IdType,
        dlc: u8
    },
    ErrorFrame {
        can_id: u32,
        id_type: IdType
    }
}


impl CanFrame {
    pub fn new_data_frame(can_id: u32, id_type: IdType, data: &[u8]) -> CanFrame {
        // mask CAN-Id based on the ID type
        let canid = can_id & match id_type {
            IdType::Standard => STANDARD_FRAME_ID_MASK,
            IdType::Extended => EXTENDED_FRAME_ID_MASK,
        };

        // allocate space for the CAN data and copy over from the provided slice
        let mut candata = [0u8; 8];
        for (cd, d) in candata.as_mut().into_iter().zip(data.into_iter()) {
            *cd = *d;
        }

        // limit the dlc to the interval [0,8]
        let dlc = if data.len() > 8 {
            8
        } else {
            data.len() as u8
        };

        // construct the data frame
        CanFrame::DataFrame { can_id: canid, id_type, dlc, data: candata }
    }

    pub fn new_remote_frame(can_id: u32, id_type: IdType, dlc: u8) -> CanFrame {
        let canid = can_id & match id_type {
            IdType::Standard => STANDARD_FRAME_ID_MASK,
            IdType::Extended => EXTENDED_FRAME_ID_MASK,
        };

        // limit the dlc to the interval [0,8]
        let dlc = if dlc > 8 {
            8u8
        } else {
            dlc
        };   

        // construct the data frame
        CanFrame::RemoteFrame { can_id: canid, id_type, dlc }
    }
}

impl Debug for CanFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanFrame::DataFrame { can_id, id_type, dlc, data } => {
                let dlc = *dlc as usize;
                let mut debugstruct = f.debug_struct("DataFrame");
                match id_type {
                    IdType::Standard => debugstruct.field("can_id", &format_args!("{:#03X}", can_id)),
                    IdType::Extended => debugstruct.field("can_id", &format_args!("{:#08X}", can_id)),
                };
                debugstruct.field("id_type", id_type);
                debugstruct.field("dlc", &dlc);
                debugstruct.field("data", &format_args!("{:02X?}", &data.as_slice()[..dlc]));
                debugstruct.finish()
            },
            CanFrame::RemoteFrame { can_id, id_type, dlc } => {
                let dlc = *dlc as usize;
                let mut debugstruct = f.debug_struct("Remote");
                match id_type {
                    IdType::Standard => debugstruct.field("can_id", &format_args!("{:#03X}", can_id)),
                    IdType::Extended => debugstruct.field("can_id", &format_args!("{:#08X}", can_id)),
                };
                debugstruct.field("id_type", id_type);
                debugstruct.field("dlc", &dlc);
                debugstruct.finish()
            },
            CanFrame::ErrorFrame { can_id, id_type } => {
                let mut debugstruct = f.debug_struct("ErrorFrame");
                match id_type {
                    IdType::Standard => debugstruct.field("can_id", &format_args!("{:#03X}", can_id)),
                    IdType::Extended => debugstruct.field("can_id", &format_args!("{:#08X}", can_id)),
                };
                debugstruct.field("id_type", id_type);
                debugstruct.finish()
            },
        }
    }
}