use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("64HUFWsdzNjn7Xan2jNwFC8S7xocbAHmtbaUUYZKXMRU");

#[program]
pub mod bridge {
    use super::*;
    pub fn freeze_token(ctx: Context<FreezeToken>, amount: u64) ->Result<()> {
        msg!("starting tokens: {}", ctx.accounts.sender_token.amount);
        token::transfer(ctx.accounts.transfer_ctx(), amount)?;
        ctx.accounts.sender_token.reload()?;
        emit!(MyEvent {
        data: 5
    });
        msg!("remaining tokens: {}", ctx.accounts.sender_token.amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FreezeToken<'info> {
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub escrow_ata: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[event]
pub struct MyEvent {
    pub data: u64

}


impl<'info> FreezeToken<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.sender_token.to_account_info(),
                to: self.escrow_ata.to_account_info(),
                authority: self.sender.to_account_info(),
            },
        )
    }
}
