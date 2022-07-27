use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

declare_id!("8SdY5Ysr3FMonohSeS1DNRnknviN7cVwp26ZwDj5ido1");

#[program]
pub mod bridge {
    use super::*;
    pub fn freeze_token(ctx: Context<FreezeToken>, amount: u64) ->Result<()> {
       // let EscrowAccount  = &mut ctx.accounts.escrow.key();
        const ESCROW_PDA_SEED: &[u8] = b"escrow";
      // msg!("{}",ctx.accounts.escrow.to_account_info().key());

        msg!("starting tokens: {}", ctx.accounts.sender_token.amount);
         let (vault_authority, _vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);

        token::set_authority(
            ctx.accounts.into_set_authority_context(),
            AuthorityType::AccountOwner,
            Some(vault_authority),
        )?;
        
          token::transfer(
            ctx.accounts.into_transfer_to_pda_context(),
            amount,
        )?;
        Ok(())
    }


    pub fn release_token(ctx: Context<ReleaseToken>, amount: u64) ->Result<()> {
       
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
   #[account(mut, signer)]
    pub sender: AccountInfo<'info>,
    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>,

   pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = [b"token-seed".as_ref(),mint.key().as_ref()],
        bump,
        payer = sender,
        token::mint = mint,
        token::authority = sender,
    )]
    pub vault_account: Account<'info, TokenAccount>,    // pub escrow: Account<'info,TokenAccount>,
    
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ReleaseToken<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>,
  
    #[account(mut)]
    pub escrow_ata: Account<'info,TokenAccount>,
    pub mint: Account<'info, Mint>,
       /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[event]
pub struct MyEvent {
    pub data: u64

}

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub mint_ata: Pubkey,
    pub mint : Pubkey,
}


impl<'info> FreezeToken<'info> {
   
     fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .sender_token
                .to_account_info()
                .clone(),
            to: self.vault_account.to_account_info().clone(),
            authority: self.sender.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
    fn into_set_authority_context(&self) -> CpiContext<'_, '_, '_, 'info, SetAuthority<'info>> {
        let cpi_accounts = SetAuthority {
            account_or_mint: self.vault_account.to_account_info().clone(),
            current_authority: self.sender.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}

    impl<'info> ReleaseToken<'info> {
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
