use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

pub use errors::BrwryError;
pub use instructions::*;
pub use state::*;

declare_id!("Brwry11111111111111111111111111111111111111");

#[program]
pub mod brwry_cellar {
    use super::*;

    pub fn create_cask(
        ctx: Context<CreateCask>,
        params: CreateCaskParams,
    ) -> Result<()> {
        instructions::create_cask::handler(ctx, params)
    }
