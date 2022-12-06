#[macro_use]
mod abort;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use fvm_shared::METHOD_CONSTRUCTOR;
use fvm_sdk::NO_DATA_BLOCK_ID;
use fvm_ipld_encoding::{DAG_CBOR, RawBytes};

#[derive(FromPrimitive)]
#[repr(u64)]
pub enum Method {
    Constructor = METHOD_CONSTRUCTOR,
    CustodyMiner,
}

#[no_mangle]
pub fn invoke(params: u32) -> u32 {
    let method = fvm_sdk::message::method_number();
    let ret: Option<RawBytes> = match FromPrimitive::from_u64(method) {
        Some(Method::Constructor) => abort!(USR_UNHANDLED_MESSAGE, "not implemented"),
        Some(Method::CustodyMiner) => abort!(USR_UNHANDLED_MESSAGE, "not implemented"),
        _ => abort!(USR_UNHANDLED_MESSAGE, "unrecognized method"),
    };

    match ret {
        None => NO_DATA_BLOCK_ID,
        Some(v) => match fvm_sdk::ipld::put_block(DAG_CBOR, v.bytes()) {
            Ok(id) => id,
            Err(err) => abort!(USR_SERIALIZATION, "failed to store return value: {}", err),
        },
    }
}

#[cfg(test)]
mod tests {
}
