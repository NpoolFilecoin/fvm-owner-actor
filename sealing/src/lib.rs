#[macro_use]
extern crate abort;

use fvm_ipld_encoding::RawBytes;

pub fn sealing_vote(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}
