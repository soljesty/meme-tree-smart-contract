use crate::constants::TREASURY_FEE;
use crate::state::GlobalState;
use crate::validator::balanceValidator;
use anchor_lang::system_program::transfer;
use anchor_lang::{prelude::*, system_program::Transfer};

#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(
        init,
        space = 8 + 96,
        payer = signer,
        seeds = [signer.key().as_ref(), my_token.key().as_ref()],
        bump,
    )]
    pub wallet_info: Account<'info, GlobalState>,
    #[account(
        mut,
        seeds = [b"wallet",signer.key().as_ref(), my_token.key().as_ref()],
        bump,
    )]
    /// CHECK
    pub marketing_wallet: AccountInfo<'info>,
    ///CHECK
    pub my_token: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK
    pub treasury: AccountInfo<'info>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeContext<'info> {
    pub fn process(&mut self, treasury_wallet: Pubkey) -> Result<()> {
        self.wallet_info.marketing_wallet = self.marketing_wallet.clone().key();
        self.wallet_info.treasury_wallet = treasury_wallet.clone();
        self.wallet_info.token_addr = self.my_token.clone().key();

        msg!(
            " ===> {} , {} , {}",
            self.wallet_info.marketing_wallet,
            self.wallet_info.treasury_wallet,
            self.wallet_info.token_addr
        );
        msg!("PDA : {:?}", self.wallet_info.to_account_info());

        balanceValidator(self.signer.to_account_info());

        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.signer.to_account_info(),
                    to: self.treasury.to_account_info(),
                },
            ),
            TREASURY_FEE as u64,
        )?;

        msg!(
            "After Lamport is ==========> {}",
            self.treasury.get_lamports()
        );

        Ok(())
    }
}
