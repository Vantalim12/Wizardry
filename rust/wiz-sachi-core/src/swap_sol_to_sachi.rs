use crate::utils::{create_rpc_client, get_sol_balance};
use crate::Config;
use anyhow::Context;
use reqwest::Client;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    transaction::Transaction,
};

#[derive(serde::Deserialize)]
struct JupiterQuote {
    #[serde(rename = "inAmount")]
    in_amount: String,
    #[serde(rename = "outAmount")]
    out_amount: String,
    #[serde(rename = "priceImpactPct")]
    price_impact: f64,
}

#[derive(serde::Deserialize)]
struct JupiterSwapResponse {
    swap_transaction: String,
}

/// Swap SOL to SACHI tokens using Jupiter aggregator
pub async fn swap_sol_to_sachi(config: &Config) -> anyhow::Result<f64> {
    let client = create_rpc_client(&config.rpc_endpoint);
    let keypair = crate::load_keypair(&config.keypair_path)?;
    
    // Check current SOL balance
    let balance = get_sol_balance(&client, &keypair.pubkey())
        .await
        .context("Failed to get SOL balance")?;
    
    let balance_sol = (balance as f64) / 1_000_000_000.0;
    log::info!("Current SOL balance: {}", balance_sol);
    
    // Reserve SOL for transactions
    let sol_to_swap = balance_sol - config.min_sol_reserve;
    
    if sol_to_swap <= 0.0 {
        log::info!("Insufficient SOL to swap (reserve requirement not met)");
        return Ok(0.0);
    }
    
    log::info!("Swapping {} SOL to SACHI", sol_to_swap);
    
    // Get quote from Jupiter
    let jupiter_client = Client::new();
    let quote_url = format!(
        "{}/quote?inputMint=So11111111111111111111111111111111111111112&outputMint={}&amount={}&slippageBps=50",
        config.jupiter_api_url,
        config.sachi_token_mint,
        (sol_to_swap * 1_000_000_000.0) as u64
    );
    
    log::info!("Fetching quote from Jupiter: {}", quote_url);
    
    let quote: JupiterQuote = jupiter_client
        .get(&quote_url)
        .send()
        .await
        .context("Failed to get Jupiter quote")?
        .json()
        .await
        .context("Failed to parse Jupiter quote")?;
    
    log::info!("Quote received: {} SACHI (price impact: {:.2}%)", 
        (quote.out_amount.parse::<u64>()? as f64) / 1_000_000_000.0,
        quote.price_impact
    );
    
    // For now, return the amount that would be swapped
    // In production, you would:
    // 1. Use Jupiter's swap API to get the transaction
    // 2. Sign and send the transaction
    // 3. Confirm the transaction
    
    log::info!("Placeholder: Would execute swap transaction via Jupiter");
    
    Ok(sol_to_swap)
}

