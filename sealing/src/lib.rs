#[macro_use]
extern crate abort;

use fvm_ipld_encoding::RawBytes;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct Sealing {
    pub approved_percent: u32,
    pub rejected_percent: u32,
}

pub fn sealing_vote(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}
