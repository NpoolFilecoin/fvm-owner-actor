#[macro_use]
extern crate abort;

use fvm_ipld_encoding::RawBytes;
use fvm_sdk as sdk;

#[derive(Debug)]
pub enum ParamsError {

}

pub fn deserialize<T: for <'de>serde::Deserialize<'de>>(params: u32) -> Result<T, ParamsError> {
    let params = match sdk::message::params_raw(params) {
        Ok(params) => params.1,
        Err(err) => abort!(USR_SERIALIZATION, "{}", err),
    };
    let params = RawBytes::new(params);
    let params: T = match params.deserialize() {
        Ok(params) => params,
        Err(err) => abort!(USR_SERIALIZATION, "{}", err)
    };
    Ok(params)
}
