use wiz_sachi_core::create_pool::create_meteora_pool;
use wiz_sachi_core::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    log::info!("Creating Meteora DLMM pool for WIZ/SOL...");
    
    let config = Config::from_env()?;
    
    match create_meteora_pool(&config).await {
        Ok(pool_address) => {
            log::info!("Pool created successfully at: {}", pool_address);
            Ok(())
        }
        Err(e) => {
            log::error!("Pool creation failed: {}", e);
            Err(e)
        }
    }
}

