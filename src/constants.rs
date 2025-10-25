use std::env::var;

use alloy::primitives::{Address, address};

const USDC_ADDRESS: &str = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913";
const DAI_ADDRESS: &str = "0x50c5725949A6F0c72E6C4a641F24049A917DB0Cb";
const WETH_ADDRESS: &str = "0x4200000000000000000000000000000000000006";

pub const TOKEN_ADDRESSES: [&str; 2] = [USDC_ADDRESS, DAI_ADDRESS];

pub const Z_QUOTER_ADDRESS: Address = address!("0x772E2810A471dB2CC7ADA0d37D6395476535889a");

pub fn private_key() -> String {
    var("PRIVATE_KEY").expect("env PRIVATE_KEY missing")
}

pub fn rpc_url() -> String {
    var("RPC_URL").expect("env RPC_URL missing")
}
