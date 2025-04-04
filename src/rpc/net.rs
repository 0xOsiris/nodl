use alloy_provider::ext::NetApi;
use clap::Parser;

use crate::{cli::RpcArgs, provider::NodlProviderBuilder};

/// CLI arguments for `cast tx-pool`.
#[derive(Debug, Parser)]
pub enum NetSubcommands {
    /// Returns a bool indicating whether or not the node is listening for network connections.
    Listening {
        #[command(flatten)]
        args: RpcArgs,
    },
    /// Returns the number of peers connected to the node.
    PeerCount {
        #[command(flatten)]
        args: RpcArgs,
    },
    /// Returns the network ID (e.g. 1 for mainnet)
    Version {
        #[command(flatten)]
        args: RpcArgs,
    },
}

impl NetSubcommands {
    pub async fn run(self) -> eyre::Result<String> {
        let res = match self {
            Self::Listening { args } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.net_listening().await?)?
            }
            Self::PeerCount { args } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.net_peer_count().await?)?
            }
            Self::Version { args } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.net_version().await?)?
            }
        };

        Ok(res)
    }
}
