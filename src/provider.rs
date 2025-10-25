use std::str::FromStr;

use alloy::{
    providers::{ProviderBuilder, fillers::FillProvider},
    rpc::client::ClientBuilder,
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};

use crate::constants::{private_key, rpc_url};

pub fn provider_instance() -> FillProvider<
    alloy::providers::fillers::JoinFill<
        alloy::providers::fillers::JoinFill<
            alloy::providers::Identity,
            alloy::providers::fillers::JoinFill<
                alloy::providers::fillers::GasFiller,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::BlobGasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::NonceFiller,
                        alloy::providers::fillers::ChainIdFiller,
                    >,
                >,
            >,
        >,
        alloy::providers::fillers::WalletFiller<alloy::network::EthereumWallet>,
    >,
    alloy::providers::RootProvider,
> {
    let signer: PrivateKeySigner = private_key().parse().unwrap();

    let rpc_url = Url::from_str(&rpc_url()).unwrap();

    let client = ClientBuilder::default().http(rpc_url);

    let provider = ProviderBuilder::new().wallet(signer).connect_client(client);

    provider
}
