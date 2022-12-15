#[macro_use]
extern crate abort;

use fvm_ipld_encoding::RawBytes;

/// Transfer nft to somebody then also transfer the amount benefit to target user
pub fn transfer_nft(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

/// Cancel nft transfer within timeout
pub fn cancel_transfer_nft(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

/// Approve nft from somebody then get tbe amount benefit
pub fn approve_nft(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

