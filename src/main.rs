mod tmdb;
mod cli;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "cli" {
        cli::cli().await;
    } else {
        println!("Usage: {} cli", args[0]);
    }
}
