use wiz_sachi_core::swap_sol_to_sachi::swap_sol_to_sachi;
use wiz_sachi_core::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    log::info!("Starting SOL to SACHI swap...");
    
    let config = Config::from_env()?;
    
    match swap_sol_to_sachi(&config).await {
        Ok(amount_swapped) => {
            log::info!("Swap completed successfully: {} SOL", amount_swapped);
            Ok(())
        }
        Err(e) => {
            log::error!("Swap failed: {}", e);
            Err(e)
        }
    }
}

