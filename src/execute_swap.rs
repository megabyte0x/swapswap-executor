use std::str::FromStr;

use crate::constants::{USDC_ADDRESS, Z_QUOTER_ADDRESS};
use crate::interfaces::{IzQuoter::buildBestSwapReturn, *};
use crate::provider::provider_instance;

use alloy::{
    eips::BlockId,
    primitives::{Address, U256},
    providers::Provider,
};

async fn execute(swapswap_contract: Address, quote_result: buildBestSwapReturn) {
    let swapswap_instance = ISwapSwap::new(swapswap_contract, provider_instance());

    let estimate_gas_execute_swap = swapswap_instance
        .executeCallDataSwap(quote_result.callData.clone(), quote_result.msgValue)
        .value(quote_result.msgValue)
        .estimate_gas()
        .await;

    match estimate_gas_execute_swap {
        Ok(gas_amounnt) => {
            let execute_swap = swapswap_instance
                .executeCallDataSwap(quote_result.callData, quote_result.msgValue)
                .value(quote_result.msgValue)
                .gas(gas_amounnt + 1000);

            let execute_swap_tx = execute_swap.send().await;

            match execute_swap_tx {
                Ok(v) => {
                    println!("tx hash: {:#?}", v.tx_hash())
                }
                Err(e) => println!("execution error: {:#?}", e),
            }
        }
        Err(e) => println!("gas estimation error: {}", e),
    }
}

pub async fn execute_swap(
    token_in: Address,
    swapswap_contract: Address,
    user: Address,
    swap_amount: U256,
) {
    // Initialize a signer with a private key

    let block = provider_instance()
        .get_block(BlockId::latest())
        .await
        .unwrap()
        .unwrap();

    let timestamp = block.header.timestamp;
    let slippage_bps = U256::from(500);
    let exact_out = false;
    let deadline = U256::from(timestamp + 1800);

    let z_quoter_instance = IzQuoter::new(Z_QUOTER_ADDRESS, provider_instance());

    let swapswap_instance = ISwapSwap::new(swapswap_contract, provider_instance());

    let i_token = swapswap_instance.i_token().call().await.unwrap();

    let token_out: Address;

    if i_token == token_in {
        token_out = Address::from_str(USDC_ADDRESS).unwrap();
    } else {
        token_out = i_token;
    }

    println!(
        "getting quote for user {} for token {} from {} token with token amount {} ",
        user, token_out, token_in, swap_amount
    );

    let quote_gen_call = z_quoter_instance
        .buildBestSwap(
            user,
            exact_out,
            token_in,
            token_out,
            swap_amount,
            slippage_bps,
            deadline,
        )
        .call()
        .await;

    match quote_gen_call {
        Ok(quote_result) => {
            execute(swapswap_contract, quote_result).await;
        }
        Err(e) => println!("quote gen failed: {:?}", e),
    }
}
