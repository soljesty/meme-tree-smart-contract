use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod validator;

pub use instructions::*;

declare_id!("F3MTcGwPoqES7a6X41mYnSAjad74jVGr8NsPuumePJLR");

#[program]
pub mod memetree {
    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>, treasury_wallet: Pubkey) -> Result<()> {
        ctx.accounts.process(treasury_wallet);
        Ok(())
    }

    pub fn withdraw(ctx: Context<WithdrawContext>, amount: u64) -> Result<()> {
        let bump = &[ctx.bumps.marketing_wallet];
        ctx.accounts.process(amount.try_into().unwrap(), bump);
        Ok(())
    }
}
