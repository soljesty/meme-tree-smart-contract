use crate::constants::{TARGET_WRATE, TREASURY_FEE, TREASURY_WRATE};
use crate::state::GlobalState;
use crate::validator::balanceValidator;
use anchor_lang::solana_program::program::invoke_signed;
use anchor_lang::solana_program::system_instruction;
use anchor_lang::system_program::transfer;
use anchor_lang::{prelude::*, system_program::Transfer};

#[derive(Accounts)]
pub struct WithdrawContext<'info> {
    #[account(
        mut,
        seeds = [b"wallet",signer.key().as_ref(), my_token.key().as_ref()],
        bump
    )]
    /// CHECK
    pub marketing_wallet: AccountInfo<'info>,
    ///CHECK
    pub my_token: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK
    pub target_wallet: SystemAccount<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawContext<'info> {
    pub fn process(&mut self, amount: i32, bumps: &[u8]) -> Result<()> {
        let binding = self.my_token.key();
        let signer_binding = self.signer.key();

        let seeds: &[&[u8]] = &[b"wallet", signer_binding.as_ref(), binding.as_ref(), bumps];
        let signer_seeds = &[&seeds[..]];

        balanceValidator(self.marketing_wallet.clone());
        msg!("=========> {:?}", &self.marketing_wallet.to_account_info());

        let transfer_ix = system_instruction::transfer(
            &self.marketing_wallet.key,
            self.target_wallet.key,
            (TARGET_WRATE * amount as f32) as u64,
        );

        invoke_signed(
            &transfer_ix,
            &[
                self.marketing_wallet.to_account_info(),
                self.target_wallet.to_account_info(),
            ],
            signer_seeds,
        )?;

        let transfer_ix = system_instruction::transfer(
            &self.marketing_wallet.key,
            self.treasury.key,
            (TREASURY_WRATE * amount as f32) as u64,
        );

        invoke_signed(
            &transfer_ix,
            &[
                self.marketing_wallet.to_account_info(),
                self.treasury.to_account_info(),
            ],
            signer_seeds,
        )?;

        msg!(
            "After Lamport is ==========> {} , {} , {}",
            self.treasury.get_lamports(),
            (TARGET_WRATE * amount as f32) as u64,
            (TREASURY_WRATE * amount as f32) as u64
        );

        Ok(())
    }
}
