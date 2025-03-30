use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachain_wetee_runtime as runtime;
use runtime::{AccountId, AuraId, EXISTENTIAL_DEPOSIT};
use sc_service::ChainType;
use sp_core::crypto::UncheckedInto;

use crate::chain_spec::*;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;
const PARACHAIN_ID: u32 = 3416;
pub fn polkadot_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "WTE".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

    #[allow(deprecated)]
    ChainSpec::builder(
        runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
        Extensions {
            relay_chain: "polkadot".into(),
            para_id: PARACHAIN_ID,
        },
    )
    .with_name("WeTEE Polkadot")
    .with_id("wetee_polkadot")
    .with_chain_type(ChainType::Live)
    .with_genesis_config_patch(polkadot_genesis(
        // initial collators.
        vec![(
            // 5EEitgFfpJVaV3CudoqHJW8or2eFJFpUfdJ51WdkXitZVuP9
            hex!["601c67623077faae301d9593535f86d94b11ead91b2bafe6593a316adb2b4900"].into(),
            hex!["601c67623077faae301d9593535f86d94b11ead91b2bafe6593a316adb2b4900"]
                .unchecked_into(),
        )],
        vec![
            // 5EEitgFfpJVaV3CudoqHJW8or2eFJFpUfdJ51WdkXitZVuP9
            hex!["601c67623077faae301d9593535f86d94b11ead91b2bafe6593a316adb2b4900"].into(),
        ],
        // 5EEitgFfpJVaV3CudoqHJW8or2eFJFpUfdJ51WdkXitZVuP9
        hex!["601c67623077faae301d9593535f86d94b11ead91b2bafe6593a316adb2b4900"].into(),
        PARACHAIN_ID.into(),
    ))
    .with_protocol_id("WeTEE")
    .with_properties(properties)
    .build()
}

fn polkadot_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    _endowed_accounts: Vec<AccountId>,
    root: AccountId,
    id: ParaId,
) -> serde_json::Value {
    serde_json::json!({
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
