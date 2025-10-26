pub mod collect_fees;
pub mod create_pool;
pub mod distribute_tokens;
pub mod swap_sol_to_sachi;
pub mod utils;

use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use std::str::FromStr;

/// Load a keypair from a JSON file
pub fn load_keypair(path: &str) -> anyhow::Result<Keypair> {
    let content = std::fs::read_to_string(path)?;
    let bytes: Vec<u8> = serde_json::from_str(&content)?;
    Ok(Keypair::from_bytes(&bytes)?)
}

/// Parse pubkey from string
pub fn parse_pubkey(s: &str) -> anyhow::Result<Pubkey> {
    Ok(Pubkey::from_str(s)?)
}

/// Environment configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub rpc_endpoint: String,
    pub keypair_path: String,
    pub keypair_distro_path: String,
    pub wiz_token_mint: Pubkey,
    pub sachi_token_mint: Pubkey,
    pub meteora_pool_address: Option<Pubkey>,
    pub jupiter_api_url: String,
    pub min_sol_reserve: f64,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        use std::env;
        
        Ok(Self {
            rpc_endpoint: env::var("RPC_ENDPOINT")?,
            keypair_path: env::var("KEYPAIR_PATH")?,
            keypair_distro_path: env::var("KEYPAIR_DISTRO_PATH")?,
            wiz_token_mint: parse_pubkey(&env::var("WIZ_TOKEN_MINT")?)?,
            sachi_token_mint: parse_pubkey(&env::var("SACHI_TOKEN_MINT")?)?,
            meteora_pool_address: env::var("METEORA_POOL_ADDRESS")
                .ok()
                .and_then(|s| parse_pubkey(&s).ok()),
            jupiter_api_url: env::var("JUPITER_API_URL")
                .unwrap_or_else(|_| "https://quote-api.jup.ag/v6".to_string()),
            min_sol_reserve: env::var("MIN_SOL_RESERVE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1.0),
        })
    }
}

