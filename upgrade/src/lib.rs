#[macro_use]
extern crate abort;

use fvm_ipld_encoding::RawBytes;

pub fn upgrade_initialize(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

pub fn upgrade_vote(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

pub fn upgrade_finalize(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

