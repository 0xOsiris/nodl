use alloy_primitives::Address;
use alloy_provider::ext::TxPoolApi;
use clap::Parser;

use crate::{cli::RpcArgs, provider::NodlProviderBuilder};

/// CLI arguments for `cast tx-pool`.
#[derive(Debug, Parser)]
pub enum TxPoolSubcommands {
    /// Fetches the content of the transaction pool.
    Content {
        #[command(flatten)]
        args: RpcArgs,
    },
    /// Fetches the content of the transaction pool filtered by a specific address.
    ContentFrom {
        /// The Signer to filter the transactions by.
        #[arg(short, long)]
        from: Address,
        #[command(flatten)]
        args: RpcArgs,
    },
    /// Fetches a textual summary of each transaction in the pool.
    Inspect {
        #[command(flatten)]
        args: RpcArgs,
    },
    /// Fetches the current status of the transaction pool.
    Status {
        #[command(flatten)]
        args: RpcArgs,
    },
}

impl TxPoolSubcommands {
    pub async fn run(self) -> eyre::Result<String> {
        let res = match self {
            Self::Content { args } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.txpool_content().await?)?
            }
            Self::ContentFrom { from, args } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.txpool_content_from(from).await?)?
            }
            Self::Inspect { args } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.txpool_inspect().await?)?
            }
            Self::Status { args } => {
                let provider = NodlProviderBuilder::default()
                    .jwt(args.jwt)
                    .url(args.rpc_url)
                    .build()?;
                serde_json::to_string_pretty(&provider.txpool_status().await?)?
            }
        };

        Ok(res)
    }
}
