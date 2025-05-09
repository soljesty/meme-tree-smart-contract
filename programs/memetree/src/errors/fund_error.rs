use anchor_lang::prelude::*;

#[error_code]
pub enum FundError {
    #[msg("Money is insufficient for transaction")]
    InsufficiencyError,
}
