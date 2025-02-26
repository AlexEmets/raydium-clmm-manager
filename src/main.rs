pub mod raydium_interaction;
pub mod cli_parser;
pub mod client_config;
pub mod instructions;
pub mod utils;

use cli_parser::{Cli, LiquidityCommand};
use clap::Parser;
use raydium_interaction::{add_liquidity, remove_liquidity, create_pool};
use client_config::load_cfg;
use anyhow::{format_err, Result};
use solana_sdk::signature::Keypair;
use solana_client::{
    rpc_client::RpcClient,
};

/// Entry point
fn main() {
    let cli = Cli::parse();
    println!("Starting...");
    let pool_config_name = "raydium_config.ini";
    let pool_config = load_cfg(&pool_config_name.to_string()).unwrap();
    println!("Loaded client config: {:?}", pool_config);

    // Admin and cluster params.
    let payer = utils::read_keypair_file(&pool_config.payer_path).expect("Payer keypair not found");
    let admin = utils::read_keypair_file(&pool_config.admin_path).expect("Admin keypair not found");
    // solana rpc client
    let rpc_client = RpcClient::new(pool_config.http_url.to_string());

        
    match cli.command {
        LiquidityCommand::CreatePool { 
            config_index, 
            price, 
            mint0, 
            mint1, 
            open_time 
        } => {
            println!(
                "Creating pool with config index {}, price {:.2}, mint0: {}, mint1: {}, open time: {}",
                config_index, price, mint0, mint1, open_time
            );
            raydium_interaction::create_pool(&pool_config, config_index, price, mint0, mint1, open_time);
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
            remove_liquidity(pool);
        }

    }
}
