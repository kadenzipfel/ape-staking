use ethers::prelude::*;
use ethers_flashbots::FlashbotsMiddleware;
use gumdrop::Options;
use reqwest::Url;
use serde_json::{Map, Value};
use std::{convert::TryFrom, sync::Arc, time::Duration};
use tracing::{info, subscriber};
use tracing_subscriber::FmtSubscriber;

use crate::{bindings::*, constants::*};

mod bindings;
mod constants;

// CLI Options
#[derive(Debug, Options, Clone)]
pub struct Opts {
    help: bool,

    #[options(help = "RPC http endpoint")]
    rpc_url: String,

    #[options(help = "polling interval (ms)", default = "1000")]
    interval: u64,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::new();
    subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let opts = Opts::parse_args_default_or_exit();

    // Initialize provider
    let provider =
        Provider::<Http>::try_from(&opts.rpc_url).expect("Failed to initialize provider");

    run(opts, provider).await?;

    Ok(())
}

async fn run<P: JsonRpcClient + 'static>(opts: Opts, provider: Provider<P>) -> anyhow::Result<()> {
    info!("Starting...");
    // let provider = provider.interval(Duration::from_millis(opts.interval));
    // let client = FlashbotsMiddleware::new(
    //     provider,
    //     Url::parse("https://relay.flashbots.net")?,
    //     bundle_signer.clone(),
    // );
    // let client = SignerMiddleware::new_with_provider_chain(client, wallet.clone()).await?;
    // let client = Arc::new(client);

    info!("Node: {}", opts.rpc_url);

    let static_provider =
        Arc::new(Provider::<Http>::try_from(&opts.rpc_url).expect("Failed to initialize provider"));

    let staking = Staking::new(
        STAKING_CONTRACT.parse::<Address>().unwrap(),
        static_provider.clone(),
    );

    let position = staking
        .nft_position(
            U256::from_dec_str("2").unwrap(),
            U256::from_dec_str("9881").unwrap(),
        )
        .call()
        .await
        .unwrap();

    info!("Position: {:?}", position);

    let mut positions = Map::new();

    let mut collections: Vec<Value> = vec![];
    for i in 1..3 {
        let mut collection = Map::new();

        match i {
            1 => collection.insert(String::from("name"), String::from("BAYC").into()),
            2 => collection.insert(String::from("name"), String::from("MAYC").into()),
            3 => collection.insert(String::from("name"), String::from("BAKC").into()),
            _ => None,
        };

        let mut collection_positions: Vec<Value> = vec![];

        for j in 0..COLLECTION_SIZES[i - 1] {
            let mut position = Map::new();

            position.insert(String::from("main_id"), i.into());
            position.insert(String::from("nft_id"), j.into());

            let result = staking
                .nft_position(
                    U256::from_dec_str(&i.to_string()).unwrap(),
                    U256::from_dec_str(&j.to_string()).unwrap(),
                )
                .call()
                .await
                .unwrap();

            position.insert(String::from("staked_amount"), result.0.to_string().into());
            position.insert(String::from("rewards_debt"), result.1.to_string().into());

            info!("Position: {:?}", position);

            collection_positions.push(position.into());
        }

        collection.insert(String::from("positions"), collection_positions.into());
        collections.push(collection.into());
    }

    positions.insert(String::from("collections"), collections.into());

    std::fs::write(
        "positions.json",
        serde_json::to_string_pretty(&positions).unwrap(),
    )
    .unwrap();

    Ok(())
}
