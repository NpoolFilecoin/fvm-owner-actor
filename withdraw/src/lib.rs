use thiserror::Error;
use fvm_shared::econ::TokenAmount;

#[derive(Error, Debug)]
pub enum WithdrawError {
}

pub fn withdraw_balance(
    amount: TokenAmount,
) -> Result<(), WithdrawError> {
    Ok(())
}

