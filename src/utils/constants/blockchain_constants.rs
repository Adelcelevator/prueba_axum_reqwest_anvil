use rust_decimal::{Decimal, MathematicalOps};

pub struct BlockchainConstants{}

impl BlockchainConstants {
    pub fn get_wei_value()->Decimal{
        Decimal::from(10_u64).powu(18)
    }
}