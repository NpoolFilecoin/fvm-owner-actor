use fvm_ipld_encoding::RawBytes;

pub fn withdraw_balance(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

