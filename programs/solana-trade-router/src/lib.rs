use anchor_lang::prelude::*;
use serum_dex::state::Market;

declare_id!("7MBTsqiPjHHt5Xmaq8jjTq93r5msdVy8rxEdyGgAnV3v");

#[program]
pub mod solana_trade_router {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn route_trade(ctx: Context<RouteTrade>) -> Result<()> {
        let market_account_info = &ctx.accounts.market;
        let program_id = ctx.program_id;

        let market = Market::load(market_account_info, program_id, false).map_err(|e| {
            msg!("Error loading market: {:?}", e);
            ProgramError::Custom(0)
        })?;

        let own_address = market.own_address;
        let vault_signer_nonce = market.vault_signer_nonce;

        msg!("Market own address: {:?}", own_address);
        msg!("Market vault signer nonce: {:?}", vault_signer_nonce);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct RouteTrade<'info> {
    pub market: AccountInfo<'info>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
}
