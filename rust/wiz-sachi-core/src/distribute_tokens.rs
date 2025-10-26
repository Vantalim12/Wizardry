use crate::utils::{create_rpc_client, get_blacklisted_addresses, get_token_accounts};
use crate::Config;
use anyhow::Context;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    transaction::Transaction,
};
use std::collections::HashMap;

/// Distribute SACHI tokens proportionally to WIZ holders
pub async fn distribute_to_holders(config: &Config) -> anyhow::Result<(usize, f64)> {
    let client = create_rpc_client(&config.rpc_endpoint);
    let keypair = crate::load_keypair(&config.keypair_distro_path)?;
    
    log::info!("Fetching all WIZ token holders...");
    
    // Get all WIZ token holders
    let holders = get_token_accounts(&client, &config.wiz_token_mint)
        .await
        .context("Failed to get WIZ holders")?;
    
    log::info!("Found {} WIZ holders", holders.len());
    
    // Filter out blacklisted addresses
    let blacklist = get_blacklisted_addresses();
    let mut filtered_holders = Vec::new();
    let mut total_supply = 0u64;
    
    for (account, amount) in holders {
        // Skip if in blacklist
        if blacklist.contains(&account) {
            continue;
        }
        
        // In production, you would also check if this is an AMM pool or DEX
        // by querying the account owner program
        
        filtered_holders.push((account, amount));
        total_supply += amount;
    }
    
    log::info!("After filtering: {} eligible holders with total supply of {}", 
        filtered_holders.len(), total_supply);
    
    if total_supply == 0 {
        log::warn!("No eligible holders found");
        return Ok((0, 0.0));
    }
    
    // Get current SACHI balance
    let sachi_balance = get_sachi_balance(&client, &keypair.pubkey(), &config.sachi_token_mint)
        .await
        .context("Failed to get SACHI balance")?;
    
    log::info!("Available SACHI to distribute: {}", sachi_balance);
    
    if sachi_balance == 0 {
        log::warn!("No SACHI available to distribute");
        return Ok((0, 0.0));
    }
    
    // Calculate proportional distributions
    let mut distributions = Vec::new();
    for (account, wiz_amount) in filtered_holders {
        let percentage = (wiz_amount as f64) / (total_supply as f64);
        let sachi_amount = ((sachi_balance as f64) * percentage) as u64;
        
        if sachi_amount > 0 {
            distributions.push((account, sachi_amount));
        }
    }
    
    log::info!("Prepared {} distributions", distributions.len());
    
    // For now, just log what would be distributed
    // In production, you would:
    // 1. Batch create transfer instructions
    // 2. Execute in parallel batches
    // 3. Handle partial failures
    
    let total_distributed: u64 = distributions.iter().map(|(_, amt)| amt).sum();
    
    log::info!("Placeholder: Would distribute {} SACHI to {} recipients", 
        (total_distributed as f64) / 1_000_000_000.0,
        distributions.len()
    );
    
    Ok((distributions.len(), (total_distributed as f64) / 1_000_000_000.0))
}

async fn get_sachi_balance(
    client: &RpcClient,
    wallet: &Pubkey,
    sachi_mint: &Pubkey,
) -> anyhow::Result<u64> {
    let token_accounts = get_token_accounts(client, sachi_mint)
        .await
        .context("Failed to get SACHI accounts")?;
    
    // Find the wallet's SACHI token account
    for (account, amount) in token_accounts {
        // In production, you would need to check if the account owner is the wallet
        // This is simplified
        if amount > 0 {
            return Ok(amount);
        }
    }
    
    Ok(0)
}

