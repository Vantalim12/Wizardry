use wiz_sachi_core::distribute_tokens::distribute_to_holders;
use wiz_sachi_core::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    log::info!("Starting token distribution...");
    
    let config = Config::from_env()?;
    
    match distribute_to_holders(&config).await {
        Ok((recipients, total_distributed)) => {
            log::info!("Distribution completed: {} recipients, {} SACHI distributed", 
                recipients, total_distributed);
            Ok(())
        }
        Err(e) => {
            log::error!("Distribution failed: {}", e);
            Err(e)
        }
    }
}

