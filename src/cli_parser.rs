use clap::Parser;
use solana_sdk::pubkey::Pubkey;
/// Solana Liquidity Manager CLI
#[derive(Parser, Debug)]
#[command(name = "solana-liq", version, about = "Manage Liquidity on Raydium CLMM", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: LiquidityCommand,
}

#[derive(Parser, Debug)]
pub enum LiquidityCommand {
    CreatePool {
        config_index: u16,
        price: f64,
        mint0: Pubkey,
        mint1: Pubkey,
        #[arg(short, long, default_value_t = 0)]
        open_time: u64,
    },
    /// Add liquidity to a specified CLMM pool with a price range
    AddLiquidity {
        /// Pool ID where liquidity should be added
        #[arg(short, long)]
        pool: String,

        /// Amount of token A to provide
        #[arg(short = None, long)]
        amount_a: f64,

        /// Amount of token B to provide
        #[arg(short = None, long)]
        amount_b: f64,

        /// Minimum price range
        #[arg(short = 'l', long)]
        min_price: f64,

        /// Maximum price range
        #[arg(short = 'u', long)]
        max_price: f64,
    },

    /// Remove all liquidity from a specified CLMM pool
    RemoveLiquidity {
        /// Pool ID from which liquidity should be removed
        #[arg(short, long)]
        pool: String,
    },
}
