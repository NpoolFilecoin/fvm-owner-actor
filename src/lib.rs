#[macro_use]
mod abort;

mod custody;
use crate::custody::*;

mod deposit;
use crate::deposit::*;

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
    ChangeWorkerAddress,
    AddControlAddress,
    DelControlAddress,
    ChangePeerID,
    ExtendSectorExpiration,
    TerminateSectors,
    WithdrawMinerBalance,
    ChangeMultiaddrs,

    /// Legacy method: add deposit
    ///  It help to submit the deposit account and amount before we can detect method 0 invocation to myself
    AddDeposit,
}

#[no_mangle]
pub fn invoke(params: u32) -> u32 {
    let method = fvm_sdk::message::method_number();
    let ret: Option<RawBytes> = match FromPrimitive::from_u64(method) {
        Some(Method::Constructor) => constructor(params),
        // Custody miner method
        Some(Method::CustodyMiner) => custody_miner(params),
        Some(Method::ChangeWorkerAddress) => change_worker_address(params),
        Some(Method::AddControlAddress) => add_control_address(params),
        Some(Method::DelControlAddress) => del_control_address(params),
        Some(Method::ChangePeerID) => change_peerid(params),
        Some(Method::ExtendSectorExpiration) => extend_sector_expiration(params),
        Some(Method::TerminateSectors) => terminate_sectors(params),
        Some(Method::WithdrawMinerBalance) => withdraw_miner_balance(params),
        Some(Method::ChangeMultiaddrs) => change_multiaddrs(params),
        // Legacy deposit method
        Some(Method::AddDeposit) => add_deposit(params),
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

/// Constructor method with initial setting of the actor
pub fn constructor(_params: u32) -> Option<RawBytes> {
    abort!(USR_UNHANDLED_MESSAGE, "not implemented")
}

#[cfg(test)]
mod tests {
}
