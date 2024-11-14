use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachain_wetee_runtime as runtime;
use runtime::{AccountId, AuraId, EXISTENTIAL_DEPOSIT, UNIT};
use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519};

use crate::chain_spec::*;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;
const TEST_PARACHAIN_ID: u32 = 4545;
pub fn paseo_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "WTE".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    #[allow(deprecated)]
    ChainSpec::builder(
        runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "paseo".into(),
            para_id: TEST_PARACHAIN_ID,
        },
    )
    .with_name("WeTEE Testnet")
    .with_id("wetee_testnet")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_patch(paseo_genesis(
        // initial collators.
        vec![(
            // 5EyMXWNZFjK5KpSgZbatRFKHk2oTimGDVTM7QtBNM3XZdvMv
            hex!["50b12ec06541ca12838ab0f6aa47f94df25c1351df28975e4ff5d8a4adaeff06"].into(),
            hex!["50b12ec06541ca12838ab0f6aa47f94df25c1351df28975e4ff5d8a4adaeff06"]
                .unchecked_into(),
        )],
        vec![
            // 5EyMXWNZFjK5KpSgZbatRFKHk2oTimGDVTM7QtBNM3XZdvMv
            hex!["50b12ec06541ca12838ab0f6aa47f94df25c1351df28975e4ff5d8a4adaeff06"].into(),
        ],
        // 5EyMXWNZFjK5KpSgZbatRFKHk2oTimGDVTM7QtBNM3XZdvMv
        hex!["50b12ec06541ca12838ab0f6aa47f94df25c1351df28975e4ff5d8a4adaeff06"].into(),
        TEST_PARACHAIN_ID.into(),
    ))
    .with_protocol_id("wetee")
    .with_properties(properties)
    .build()
}

fn paseo_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    root: AccountId,
    id: ParaId,
) -> serde_json::Value {
    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|k| (k, 100*UNIT)).collect::<Vec<_>>(),
        },
        "parachainInfo": {
            "parachainId": id,
        },
        "collatorSelection": {
            "invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
            "candidacyBond": EXISTENTIAL_DEPOSIT * 16,
        },
        "session": {
            "keys": invulnerables
                .into_iter()
                .map(|(acc, aura)| {
                    (
                        acc.clone(),                 // account id
                        acc,                         // validator id
                        template_session_keys(aura), // session keys
                    )
                })
            .collect::<Vec<_>>(),
        },
        "polkadotXcm": {
            "safeXcmVersion": Some(SAFE_XCM_VERSION),
        },
        "sudo": { "key": Some(root) }
    })
}
