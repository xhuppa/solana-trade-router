use anchor_lang::prelude::*;

declare_id!("Awjrxd8FQnGPYzHqTAJh7Af3NV5vxCkKiYt4PttMb5oy");

#[program]
pub mod solana_trade_router {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
