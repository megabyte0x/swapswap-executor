// Example of watching for new DAI transfers
// WARNING: This example doesn't account for rollbacks

mod constants;
mod execute_swap;
mod indexer;
mod interfaces;
mod provider;

use dotenvy::dotenv;

use crate::indexer::indxer;

fn main() {
    dotenv().ok();

    indxer();
}
