use crate::constants::{CBBTC_ADDRESS, TOKEN_ADDRESSES, TRANSFER_EVENT_TOPIC};
use crate::execute_swap::execute_swap;

use alloy::{
    primitives::{Address, B256},
    transports::http::reqwest::Url,
};
use hypersync_client::{Client, ClientConfig, Decoder, net_types::Query};
use std::str::FromStr;
use tokio::time::{Duration, sleep};

#[tokio::main]
pub async fn indxer() {
    env_logger::init();

    let contract_address = B256::left_padding_from(CBBTC_ADDRESS.as_slice());

    // create default client, uses eth mainnet
    let client = Client::new(ClientConfig {
        url: Some(Url::from_str("https://base.hypersync.xyz").unwrap()),
        bearer_token: None,
        http_req_timeout_millis: None,
        max_num_retries: None,
        retry_backoff_ms: None,
        retry_base_ms: None,
        retry_ceiling_ms: None,
    })
    .unwrap();

    let height = client.get_height().await.unwrap();

    println!("server height is {height}");

    // The query to run
    let mut query: Query = serde_json::from_value(serde_json::json!( {
        // start from tip since we only want new transfers
        "from_block": height,
        // The logs we want. We will also automatically get transactions and blocks relating to these logs (the query implicitly joins them).
        "logs": [
            {
                "address": TOKEN_ADDRESSES,
                // We only want transfer events
                "topics": [
                    [TRANSFER_EVENT_TOPIC],
                    [],
                    [contract_address]
                ]
            }
        ],
        // Select the fields we are interested in, notice topics are selected as topic0,1,2,3
        "field_selection": {
            "log": [
                "address",
                "data",
                "topic0",
                "topic1",
                "topic2",
                "topic3",
            ],

        }
    }))
    .unwrap();

    let decoder = Decoder::from_signatures(&[
        "Transfer(address indexed from, address indexed to, uint amount)",
    ])
    .unwrap();

    loop {
        let res = client.get(&query).await.unwrap();

        for batch in res.data.logs {
            for log in batch {
                let from_token = Address::from_slice(log.address.as_ref().unwrap().as_slice());
                let decoded_log = decoder.decode_log(&log).unwrap().unwrap();
                let amount = decoded_log.body[0].as_uint().unwrap().0;
                let from = decoded_log.indexed[0].as_address().unwrap();
                let to = decoded_log.indexed[1].as_address().unwrap();

                execute_swap(from_token, to, from, amount).await;
            }
        }

        println!("scanned up to block {}", res.next_block);

        if let Some(archive_height) = res.archive_height {
            if archive_height < res.next_block {
                // wait if we are at the head
                // notice we use explicit get_height in order to not waste data requests.
                // get_height is lighter compared to spamming data requests at the tip.
                while client.get_height().await.unwrap() < res.next_block {
                    sleep(Duration::from_secs(1)).await;
                }
            }
        }

        // continue query from next_block
        query.from_block = res.next_block;
    }
}
