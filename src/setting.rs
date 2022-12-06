use fvm_ipld_encoding::RawBytes;

/// Create a setting store with initialize setting from actor creator
pub fn setting_initialize(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

/// Update setting, normally it's invoked when a new miner custody happen
pub fn setting_update(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

