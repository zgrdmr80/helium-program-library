use anchor_lang::prelude::*;

declare_id!("hemABtqNUst4MmqsVcuN217ZzBspENbGt9uueSe5jts");

pub mod error;
pub mod instructions;
pub mod state;
pub mod token_metadata;

pub use instructions::*;
pub use state::*;

#[program]
pub mod helium_entity_manager {
  use super::*;

  pub fn initialize_hotspot_config_v0(
    ctx: Context<InitializeHotspotConfigV0>,
    args: InitializeHotspotConfigArgsV0,
  ) -> Result<()> {
    initialize_hotspot_config_v0::handler(ctx, args)
  }

  pub fn initialize_hotspot_issuer_v0(
    ctx: Context<InitializeHotspotIssuerV0>,
    args: InitializeHotspotIssuerArgsV0,
  ) -> Result<()> {
    initialize_hotspot_issuer_v0::handler(ctx, args)
  }

  pub fn issue_hotspot_v0(ctx: Context<IssueHotspotV0>, args: IssueHotspotArgsV0) -> Result<()> {
    issue_hotspot_v0::handler(ctx, args)
  }
}
