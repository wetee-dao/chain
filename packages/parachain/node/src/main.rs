#![warn(missing_docs)]

use polkadot_sdk::*;

mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;
mod spec_dev_net;
mod spec_paseo_net;
mod spec_polkadot_net;

fn main() -> sc_cli::Result<()> {
    command::run()
}
