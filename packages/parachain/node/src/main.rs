//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]

mod chain_spec;
mod cli;
mod command;
mod rpc;
mod service;
mod spec_dev_net;
mod spec_paseo_net;

fn main() -> sc_cli::Result<()> {
    command::run()
}
