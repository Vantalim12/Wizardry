use wiz_sachi_core::collect_fees::collect_pool_fees;
use wiz_sachi_core::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    log::info!("Starting fee collection...");
    
    let config = Config::from_env()?;
    
    match collect_pool_fees(&config).await {
        Ok(_) => {
            log::info!("Fee collection completed successfully");
            Ok(())
        }
        Err(e) => {
            log::error!("Fee collection failed: {}", e);
            Err(e)
        }
    }
}

