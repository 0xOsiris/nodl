use alloy_provider::ext::AdminApi;
use clap::Parser;

use crate::{cli::RpcArgs, provider::NodlProviderBuilder};

/// CLI arguments for `cast tx-pool`.
#[derive(Debug, Parser)]
pub enum AdminSubcommands {
    /// Add the given peer to the current peer set of the node.
    AddPeer {
        #[command(flatten)]
        args: RpcArgs,

        #[arg(long, short)]
        peer: String,
    },
    /// Disconnects from a peer if the connection exists.
    /// Returns a bool indicating whether the peer was successfully removed or not.
    RemovePeer {
        #[command(flatten)]
        args: RpcArgs,
        #[arg(long, short)]
        peer: String,
    },

    /// Adds the given peer to a list of trusted peers,
    /// which allows the peer to always connect, even if there would be no room for it otherwise.
    AddTrustedPeer {
        #[command(flatten)]
        args: RpcArgs,
        #[arg(long, short)]
        peer: String,
    },

    /// Removes a remote node from the trusted peer set, but it does not disconnect it automatically.
    RemoveTrustedPeer {
        #[command(flatten)]
        args: RpcArgs,
        #[arg(long, short)]
        peer: String,
    },

    /// Returns all information known about the running node.
    NodeInfo {
        #[command(flatten)]
        args: RpcArgs,
    },
}

impl AdminSubcommands {
    pub async fn run(self) -> eyre::Result<String> {
        let res = match self {
            Self::AddPeer { args, peer } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.add_peer(&peer).await?)?
            }
            Self::RemovePeer { args, peer } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.remove_peer(&peer).await?)?
            }
            Self::AddTrustedPeer { args, peer } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.add_trusted_peer(&peer).await?)?
            }
            Self::RemoveTrustedPeer { args, peer } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.remove_trusted_peer(&peer).await?)?
            }
            Self::NodeInfo { args } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.node_info().await?)?
            }
        };

        Ok(res)
    }
}
