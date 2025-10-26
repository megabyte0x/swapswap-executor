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
