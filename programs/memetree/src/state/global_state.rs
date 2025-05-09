use anchor_lang::prelude::*;

#[account]
#[derive(Debug)]
pub struct GlobalState {
    pub marketing_wallet: Pubkey,
    pub treasury_wallet: Pubkey,
    pub token_addr: Pubkey,
}
