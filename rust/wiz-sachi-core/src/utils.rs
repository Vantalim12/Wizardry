use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    account::Account,
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Keypair,
};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

/// Initialize RPC client
pub fn create_rpc_client(rpc_url: &str) -> Arc<RpcClient> {
    Arc::new(RpcClient::new_with_commitment(
        rpc_url.to_string(),
        CommitmentConfig::confirmed(),
    ))
}

/// Get token accounts for a mint
pub async fn get_token_accounts(
    client: &RpcClient,
    mint: &Pubkey,
) -> anyhow::Result<Vec<(Pubkey, u64)>> {
    let accounts = client
        .get_program_accounts_with_config(
            &spl_token::ID,
            solana_client::rpc_config::RpcProgramAccountsConfig {
                filters: Some(vec![
                    solana_client::rpc_filter::RpcFilterType::DataSize(spl_token::state::Account::LEN as u64),
                    solana_client::rpc_filter::RpcFilterType::Memcmp(
                        solana_client::rpc_filter::RpcFilterType::new_memcmp_comparison(
                            0,
                            mint.as_ref().to_vec(),
                        ),
                    ),
                ]),
                account_config: solana_client::rpc_config::RpcAccountInfoConfig {
                    encoding: Some(solana_client::rpc_config::UiAccountEncoding::Base64),
                    data_slice: None,
                    commitment: Some(solana_sdk::commitment_config::CommitmentConfig::confirmed()),
                    min_context_slot: None,
                },
                ..Default::default()
            },
        )
        .await?;

    let mut holders = Vec::new();
    for (pubkey, account) in accounts {
        if let Ok(parsed) = spl_token::state::Account::unpack(&account.data) {
            if parsed.amount > 0 {
                holders.push((pubkey, parsed.amount));
            }
        }
    }

    Ok(holders)
}

/// Get SOL balance
pub async fn get_sol_balance(client: &RpcClient, pubkey: &Pubkey) -> anyhow::Result<u64> {
    Ok(client.get_balance(pubkey).await?)
}

/// Blacklisted addresses to exclude from distribution
pub fn get_blacklisted_addresses() -> Vec<Pubkey> {
    vec![
        // AMM/DEX program addresses
        solana_sdk::pubkey::Pubkey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap(),
        solana_sdk::pubkey::Pubkey::from_str("DXngKjkQsJN8DSYAkH5FzPRzZxKfvHqqzBUkZHFqC7mK").unwrap(),
        solana_sdk::pubkey::Pubkey::from_str("Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB").unwrap(),
    ]
}

