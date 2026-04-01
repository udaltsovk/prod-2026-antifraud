use crate::transaction::{
    merchant_category_code::TransactionMerchantCategoryCode,
    merchant_id::TransactionMerchantId,
};

pub mod filter;

#[derive(Debug)]
pub struct MerchantRiskStats {
    pub merchant_id: TransactionMerchantId,
    pub merchant_category_code: Option<TransactionMerchantCategoryCode>,
    pub tx_count: i64,
    pub gmv: f64,
    pub decline_rate: f32,
}
