//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]

mod chain_spec;
mod cli;
mod command;
mod paseo_net;
mod rpc;
mod service;
mod wetee_dev_net;

fn main() -> sc_cli::Result<()> {
    command::run()
}
