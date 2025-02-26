use crate::client_config::ClientConfig;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;

use crate::instructions::create_pool::create_pool_instr;
use crate::utils;
use raydium_amm_v3::libraries::tick_math;
use solana_sdk::program_pack::Pack;
use solana_sdk::{signature::Signer, transaction::Transaction};

pub fn add_liquidity(
    pool: String,
    _amount_a: f64,
    _amount_b: f64,
    _min_price: f64,
    _max_price: f64,
) {
    // TODO: Integrate with Raydium CLMM SDK
    println!("Liquidity added to pool {} successfully!", pool);
}

/// Mock function for removing liquidity
pub fn remove_liquidity(pool: String) {
    // TODO: Integrate with Raydium CLMM SDK
    println!("Liquidity removed from pool {} successfully!", pool);
}

/// inner function to create instruction for creating pool and send it
pub fn create_pool(
    pool_config: &ClientConfig,
    config_index: u16,
    price: f64,
    mint0: Pubkey,
    mint1: Pubkey,
    open_time: u64,
) -> anyhow::Result<()> {
    println!(
        "Creating pool with config index {}, price {:.2}, mint0: {}, mint1: {}, open time: {}",
        config_index, price, mint0, mint1, open_time
    );
    let rpc_client = RpcClient::new(pool_config.http_url.to_string());
    let payer = utils::read_keypair_file(&pool_config.payer_path)?;
    let mut price = price;
    let mut mint0 = mint0;
    let mut mint1 = mint1;

    if mint0 > mint1 {
        std::mem::swap(&mut mint0, &mut mint1);
        price = 1.0 / price;
    }

    let load_pubkeys = vec![mint0, mint1];
    let rsps = rpc_client.get_multiple_accounts(&load_pubkeys)?;

    let mint0_owner = rsps[0].clone().unwrap().owner;
    let mint1_owner = rsps[1].clone().unwrap().owner;

    let mint0_account = spl_token::state::Mint::unpack(&rsps[0].as_ref().unwrap().data).unwrap();
    let mint1_account = spl_token::state::Mint::unpack(&rsps[1].as_ref().unwrap().data).unwrap();

    let sqrt_price_x64 =
        utils::price_to_sqrt_price_x64(price, mint0_account.decimals, mint1_account.decimals);

    // get PDA address for amm_config
    let (amm_config_pubkey, __bump) = Pubkey::find_program_address(
        &[
            raydium_amm_v3::states::AMM_CONFIG_SEED.as_bytes(),
            &config_index.to_be_bytes(),
        ],
        &pool_config.raydium_v3_program,
    );
    let tick = tick_math::get_tick_at_sqrt_price(sqrt_price_x64).unwrap();
    println!(
        "tick:{}, price:{}, sqrt_price_x64:{}, amm_config_key:{}",
        tick, price, sqrt_price_x64, amm_config_pubkey
    );

    let create_pool_instr = create_pool_instr(
        &pool_config.clone(),
        amm_config_pubkey,
        mint0,
        mint1,
        mint0_owner,
        mint1_owner,
        pool_config.tickarray_bitmap_extension.unwrap(),
        sqrt_price_x64,
        open_time,
    )?;

    // send
    let signers = vec![&payer];
    let recent_hash = rpc_client.get_latest_blockhash()?;
    let txn = Transaction::new_signed_with_payer(
        &create_pool_instr,
        Some(&payer.pubkey()),
        &signers,
        recent_hash,
    );
    let signature = utils::send_txn(&rpc_client, &txn, true)?;
    println!("{}", signature);

    Ok(())
}
