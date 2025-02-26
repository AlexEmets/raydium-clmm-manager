use crate::client_config::ClientConfig; 
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
    system_program,
    sysvar,
};
use anyhow::Result;
use solana_client::rpc_client::RpcClient;
// import read_keypair_file
use crate::utils::read_keypair_file;
use anchor_client::{Client, Cluster};
use std::rc::Rc;

use raydium_amm_v3::states::{
    AMM_CONFIG_SEED, OBSERVATION_SEED, OPERATION_SEED, POOL_SEED, POOL_VAULT_SEED, POSITION_SEED,
    TICK_ARRAY_SEED,
};
use raydium_amm_v3::instruction as raydium_instruction;
use raydium_amm_v3::accounts as raydium_accounts;

pub fn create_pool_instr(   
    config: &ClientConfig,
    amm_config: Pubkey,
    token_mint_0: Pubkey,
    token_mint_1: Pubkey,
    token_program_0: Pubkey,
    token_program_1: Pubkey,
    tick_array_bitmap: Pubkey,
    sqrt_price_x64: u128,
    open_time: u64,
) -> Result<Vec<Instruction>> {
    let payer = read_keypair_file(&config.payer_path)?;
    let url = Cluster::Custom(config.http_url.clone(), config.ws_url.clone());
    // Client.
    let client = Client::new(url, Rc::new(payer));
    let program = client.program(config.raydium_v3_program)?;
    let (pool_account_key, __bump) = Pubkey::find_program_address(
        &[
            POOL_SEED.as_bytes(),
            amm_config.to_bytes().as_ref(),
            token_mint_0.to_bytes().as_ref(),
            token_mint_1.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (token_vault_0, __bump) = Pubkey::find_program_address(
        &[
            POOL_VAULT_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
            token_mint_0.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (token_vault_1, __bump) = Pubkey::find_program_address(
        &[
            POOL_VAULT_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
            token_mint_1.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let (observation_key, __bump) = Pubkey::find_program_address(
        &[
            OBSERVATION_SEED.as_bytes(),
            pool_account_key.to_bytes().as_ref(),
        ],
        &program.id(),
    );
    let instructions = program
        .request()
        .accounts(raydium_accounts::CreatePool {
            pool_creator: program.payer(),
            amm_config,
            pool_state: pool_account_key,
            token_mint_0,
            token_mint_1,
            token_vault_0,
            token_vault_1,
            observation_state: observation_key,
            tick_array_bitmap,
            token_program_0,
            token_program_1,
            system_program: system_program::id(),
            rent: sysvar::rent::id(),
        })
        .args(raydium_instruction::CreatePool {
            sqrt_price_x64,
            open_time,
        })
        .instructions()?;
    Ok(instructions)
}