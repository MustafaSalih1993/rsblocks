mod config;
mod run;
mod types;
mod utils;
mod blockmanager;

use std::env;
use std::process;

use blockmanager::BlockManager;

use lazy_static::initialize;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    initialize(&config::CONFIG);

    // if X display is not found then exit the program
    if env::var("DISPLAY").is_err() {
        eprintln!("Error: No Display Running!");
        process::exit(1);
    };

    let blocks = BlockManager::new();
    run::run(blocks).await;
    Ok(())
}
