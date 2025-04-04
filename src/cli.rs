use std::path::PathBuf;

use alloy_rpc_types_engine::JwtSecret;
use clap::Parser;

use crate::rpc::{
    admin::AdminSubcommands, net::NetSubcommands, txpool::TxPoolSubcommands, web3::Web3Subcommands,
};

/// CLI arguments for `node-kit`
#[derive(Parser, Debug)]
#[command(
    name = "nodl",
    version,
    about = "A CLI for interacting with Ethereum nodes."
)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub rpc_opts: RpcOpts,
    /// Path to dump the json output to.
    /// If not provided, the output will be printed to stdout.
    #[arg(short, long)]
    pub path: Option<PathBuf>,
}

impl Cli {
    /// Run the CLI.
    pub async fn run(self) -> eyre::Result<String> {
        let res = match self.rpc_opts {
            RpcOpts::TxPool { command } => command.run().await?,
            RpcOpts::Net { command } => command.run().await?,
            RpcOpts::Admin { command } => command.run().await?,
            RpcOpts::Web3 { command } => command.run().await?,
        };

        if let Some(path) = self.path {
            serde_json::to_writer_pretty(std::fs::File::create(path)?, &res)?;
        } else {
            println!("{:#}", res);
        }

        Ok(res)
    }
}

#[derive(Parser, Debug)]
pub enum RpcOpts {
    /// Inspect the TxPool of a node.
    #[command(visible_alias = "tp")]
    TxPool {
        #[command(subcommand)]
        command: TxPoolSubcommands,
    },
    /// Interface with the Net API of a node.
    #[command(visible_alias = "n")]
    Net {
        #[command(subcommand)]
        command: NetSubcommands,
    },
    /// Interface with the Admin API of a node.
    #[command(visible_alias = "a")]
    Admin {
        #[command(subcommand)]
        command: AdminSubcommands,
    },
    /// Interface with the Web3 API of a node.
    #[command(visible_alias = "w")]
    Web3 {
        #[command(subcommand)]
        command: Web3Subcommands,
    },
}

#[derive(Parser, Debug)]
pub struct RpcArgs {
    /// The URL of the node to connect to.
    #[arg(long, default_value = "http://localhost:8545", short)]
    pub rpc_url: String,

    /// The JWT secret for authentication.
    #[arg(long)]
    pub jwt: Option<JwtSecret>,
}
