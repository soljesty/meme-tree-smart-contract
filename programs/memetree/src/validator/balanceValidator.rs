use anchor_lang::prelude::Signer;
use anchor_lang::prelude::*;

use crate::{constants::TREASURY_FEE, errors::FundError};

pub fn balanceValidator(wallet: AccountInfo) {
    msg!("Before Lamport is ==========> {}", wallet.get_lamports());
    if wallet.get_lamports() < TREASURY_FEE as u64 {
        FundError::InsufficiencyError;
    }
}
