use alloy_provider::Provider;
use clap::Parser;

use crate::{cli::RpcArgs, provider::NodlProviderBuilder};

/// CLI arguments for `cast tx-pool`.
#[derive(Debug, Parser)]
pub enum Web3Subcommands {
    /// Returns the current client version.
    ClientVersion {
        #[command(flatten)]
        args: RpcArgs,
    },
}

impl Web3Subcommands {
    pub async fn run(self) -> eyre::Result<String> {
        let res = match self {
            Self::ClientVersion { args } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.get_client_version().await?)?
            }
        };

        Ok(res)
    }
}
