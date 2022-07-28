use anchor_lang::prelude::*;
use anchor_spl::token::{self, CloseAccount, Mint, SetAuthority, TokenAccount, Transfer};
use spl_token::instruction::AuthorityType;

declare_id!("8SdY5Ysr3FMonohSeS1DNRnknviN7cVwp26ZwDj5ido1");

#[program]
pub mod bridge {
    use super::*;
    const ESCROW_PDA_SEED: &[u8] = b"escrow";
    pub fn freeze_token(ctx: Context<FreezeToken>, amount: u64,chain_id : u8,eth_address : String) ->Result<()> {
        
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

        emit!(MyEvent {
            chain_id: chain_id,
            sender : ctx.accounts.sender.key(),
            mint : ctx.accounts.mint.key(),
            eth_address : eth_address
        });
        // msg!("{}",&eth_address);

        Ok(())
    }


    pub fn release_token(ctx: Context<ReleaseToken>, amount: u64) ->Result<()> {

        const ESCROW_PDA_SEED: &[u8] = b"escrow";
        let (_vault_authority, vault_authority_bump) =
            Pubkey::find_program_address(&[ESCROW_PDA_SEED], ctx.program_id);

        let authority_seeds = &[&ESCROW_PDA_SEED[..], &[vault_authority_bump]];

        token::transfer(
            ctx.accounts
                .into_transfer_to_receiver_context()
                .with_signer(&[&authority_seeds[..]]),
            amount
        )?;

        token::close_account(
            ctx.accounts
                .into_close_context()
                .with_signer(&[&authority_seeds[..]]),
        )?;
 
        Ok(())
    }
}

#[derive(Accounts)]
pub struct FreezeToken<'info> {
    ///CHECK : Not dangerous
   #[account(mut, signer)]
    pub sender: AccountInfo<'info>,
    #[account(mut)]
    pub sender_ata: Account<'info, TokenAccount>,

   pub mint: Account<'info, Mint>,
    #[account(
        init,
        seeds = 
        [   
            b"vault".as_ref(),
            mint.key().as_ref()
        ],
        bump,
        payer = sender,
        token::mint = mint,
        token::authority = sender,
    )]
    pub vault_account: Account<'info, TokenAccount>,   
      ///CHECK : Not dangerous
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct ReleaseToken<'info> {
    ///CHECK : Not dangerous
    #[account(mut, signer)]
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    pub sender: AccountInfo<'info>,
    #[account(mut)]
    pub receiver_ata: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub vault_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub vault_authority: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[event]
pub struct MyEvent {
    pub chain_id: u8,
    pub sender : Pubkey,
    pub mint : Pubkey,
    pub eth_address : String

}

impl<'info> FreezeToken<'info> {
   
    fn into_transfer_to_pda_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self
                .sender_ata
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

    fn into_transfer_to_receiver_context(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        let cpi_accounts = Transfer {
            from: self.vault_account.to_account_info().clone(),
            to: self.receiver_ata.to_account_info().clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }

     fn into_close_context(&self) -> CpiContext<'_, '_, '_, 'info, CloseAccount<'info>> {
        let cpi_accounts = CloseAccount {
            account: self.vault_account.to_account_info().clone(),
            destination: self.sender.clone(),
            authority: self.vault_authority.clone(),
        };
        CpiContext::new(self.token_program.clone(), cpi_accounts)
    }
}
