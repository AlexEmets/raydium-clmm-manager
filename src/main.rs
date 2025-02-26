pub mod cli_parser;
pub mod client_config;
pub mod instructions;
pub mod raydium_interaction;
pub mod utils;

use clap::Parser;
use cli_parser::{Cli, LiquidityCommand};
use client_config::load_cfg;

/// Entry point
fn main() {
    let cli = Cli::parse();
    println!("Starting...");
    let pool_config_name = "raydium_config.ini";
    let pool_config = load_cfg(&pool_config_name.to_string()).unwrap();
    println!("Loaded client config: {:?}", pool_config);

    match cli.command {
        LiquidityCommand::CreatePool {
            config_index,
            price,
            mint0,
            mint1,
            open_time,
        } => {
            println!(
                "Creating pool with config index {}, price {:.2}, mint0: {}, mint1: {}, open time: {}",
                config_index, price, mint0, mint1, open_time
            );
            let _ = raydium_interaction::create_pool(
                &pool_config,
                config_index,
                price,
                mint0,
                mint1,
                open_time,
            );
        }
        LiquidityCommand::AddLiquidity {
            pool,
            amount_a,
            amount_b,
            min_price,
            max_price,
        } => {
            println!(
                "Adding liquidity to pool {} with {:.2} A, {:.2} B, price range: {:.2} - {:.2}",
                pool, amount_a, amount_b, min_price, max_price
            );
            raydium_interaction::add_liquidity(pool, amount_a, amount_b, min_price, max_price);
        }

        LiquidityCommand::RemoveLiquidity { pool } => {
            println!("Removing all liquidity from pool {}", pool);
            raydium_interaction::remove_liquidity(pool);
        }
    }
}
