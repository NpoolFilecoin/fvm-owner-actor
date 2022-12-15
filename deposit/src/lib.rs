use fvm_shared::econ::TokenAmount;
use cid::Cid;
use thiserror::Error;

pub enum DepositError {

}

pub fn add_deposit(
    amount: TokenAmount,
    state: &Cid,
) -> Result<(), DepositError> {
    Ok(())
}

