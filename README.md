# Solana Raydium Liquidity Management CLI

## Overview
This project is a **CLI tool** for interacting with **Raydium** liquidity pools on the **Solana blockchain**. It allows users to:

- **Create a new liquidity pool**
- **Add liquidity** to an existing pool
- **Remove liquidity** from a pool

The tool reads configuration parameters from a config file and executes the specified liquidity commands on-chain.

## Features
- Uses Solana's **RPC Client** for blockchain interactions
- Reads **keypairs** securely from files
- Implements **Raydium liquidity pool operations**
- CLI parsing with `clap`


## Configuration File: `raydium_config.ini`
The tool uses an **INI configuration file** to store key Solana parameters


### Configuration Fields
- **payer_path**: Path to the private key file of the Solana payer account (for transaction fees)
- **admin_path**: Path to the admin keypair managing liquidity operations
- **http_url**: Solana RPC endpoint for blockchain interaction

## CLI Commands
### 1. **Create a Liquidity Pool**
```sh
cargo run -- create-pool --config-index <index> --price <price> --mint0 <MINT0> --mint1 <MINT1> --open-time <TIME>
```
**Arguments:**
- `config-index` - Index of the pool configuration
- `price` - Initial price of the pool
- `mint0`, `mint1` - Token mint addresses for the liquidity pair
- `open-time` - Timestamp for when the pool opens

### 2. **Add Liquidity**
```sh
cargo run -- add-liquidity --pool <POOL_ID> --amount-a <AMOUNT_A> --amount-b <AMOUNT_B> --min-price <MIN> --max-price <MAX>
```
**Arguments:**
- `pool` - Liquidity pool address
- `amount-a`, `amount-b` - Amounts of token A and B to deposit
- `min-price`, `max-price` - Price range for the liquidity position

### 3. **Remove Liquidity**
```sh
cargo run -- remove-liquidity --pool <POOL_ID>
```
**Arguments:**
- `pool` - Liquidity pool address from which to remove all liquidity

## Dependencies
The tool uses the following crates:
- `solana-sdk` - For working with Solana accounts and transactions
- `solana-client` - For interacting with the Solana blockchain
- `clap` - CLI argument parsing
- `anyhow` - Error handling

## Running the Program
1. Ensure **Rust** is installed.
2. Set up your **Solana CLI** and wallet (`solana-keygen new` if needed).
3. Create and configure `raydium_config.ini`.
4. Run commands using `cargo run -- <command>`.


