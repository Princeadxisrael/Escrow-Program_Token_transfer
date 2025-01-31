use anchor_lang::prelude::*;

declare_id!("3pYab2tSYxvvsg9Fc7YsLHPzmYBdcAgJ74ggX2CPKuKR");
pub mod state;
pub mod contexts;

pub use state::*;
pub use contexts::*;

#[program]
pub mod token_swap_escrow {

    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, amount: u64, deposit: u64) -> Result<()> {
        ctx.accounts.make(seed, amount, &ctx.bumps, deposit)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.take()?;
        ctx.accounts.close()
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()
    }
}
