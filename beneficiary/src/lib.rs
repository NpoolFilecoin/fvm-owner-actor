use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_ipld_encoding::tuple::{Deserialize_tuple, Serialize_tuple};

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct PercentBeneficiary {
    pub address: Address,
    pub percent: u32,
}

#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug)]
pub struct AmountBeneficiary {
    pub address: Address,
    pub amount: TokenAmount,
}
