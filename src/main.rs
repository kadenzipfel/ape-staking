use ethers::prelude::*;
use ethers_flashbots::FlashbotsMiddleware;
use gumdrop::Options;
use reqwest::Url;
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

    Ok(())
}
