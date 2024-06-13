### cargo.toml

```
# Local Dependencies
orml-tokens = {workspace = true}
orml-traits = {workspace = true}
pallet-contracts = {workspace = true}
pallet-insecure-randomness-collective-flip = {workspace = true}
pallet-message-queue = {workspace = true}
pallet-utility = {workspace = true}

wetee-app = {path = "../../pallets/wetee-app", default-features = false}
wetee-assets = {path = "../../pallets/wetee-assets", default-features = false}
wetee-gov = {path = "../../pallets/wetee-gov", default-features = false}
wetee-gpu = {path = "../../pallets/wetee-gpu", default-features = false}
wetee-guild = {path = "../../pallets/wetee-guild", default-features = false}
wetee-org = {path = "../../pallets/wetee-org", default-features = false}
wetee-primitives = {path = "../../primitives", default-features = false}
wetee-project = {path = "../../pallets/wetee-project", default-features = false}
wetee-runtime-api = {path = "../../pallets/wetee-rpc/runtime-api", default-features = false, optional = true}
wetee-sudo = {path = "../../pallets/wetee-sudo", default-features = false}
wetee-task = {path = "../../pallets/wetee-task", default-features = false}
wetee-treasury = {path = "../../pallets/wetee-treasury", default-features = false}
wetee-worker = {path = "../../pallets/wetee-worker", default-features = false}
```

```
  "wetee-org/runtime-benchmarks",
  "wetee-sudo/runtime-benchmarks",
  "wetee-guild/runtime-benchmarks",
  "wetee-treasury/runtime-benchmarks",
  "wetee-gov/runtime-benchmarks",
  "wetee-gpu/runtime-benchmarks",
```

```
"pallet-insecure-randomness-collective-flip/std",
"pallet-message-queue/std",
"pallet-utility/std",
"orml-traits/std",
"orml-tokens/std",
"wetee-primitives/std",
"wetee-org/std",
"wetee-sudo/std",
"wetee-gov/std",
"wetee-assets/std",
"wetee-guild/std",
"wetee-project/std",
"wetee-worker/std",
"wetee-app/std",
"wetee-task/std",
"wetee-gpu/std",
"wetee-treasury/std",
"wetee-runtime-api/std",
```

### runtime/src/lib.rs

```rust
// Import the WETEE pallet.
mod call;
pub use call::*;
mod vote;
pub use vote::*;
mod worker;
pub use worker::*;
mod wetee;
pub use wetee::*;

pub use wetee_app::Call as WeteeAppCall;
pub use wetee_assets::Call as WeteeAssetsCall;
pub use wetee_gov::Call as WeteeGovCall;
pub use wetee_gpu::Call as WeteeGpuCall;
pub use wetee_guild::Call as WeteeGuildCall;
pub use wetee_org::Call as WeteeOrgCall;
pub use wetee_project::Call as WeteeProjectCall;
pub use wetee_sudo::Call as WeteeSudoCall;
pub use wetee_task::Call as WeteeTaskCall;
pub use wetee_treasury::Call as WeteeTreasuryCall;
pub use wetee_worker::Call as WeteeWorkerCall;
// End WETEE pallet.
```

```rust
    // WETEE
    #[runtime::pallet_index(7)]
    pub type Tokens = orml_tokens;
    #[runtime::pallet_index(8)]
    pub type RandomnessCollectiveFlip = pallet_insecure_randomness_collective_flip;
    #[runtime::pallet_index(9)]
    pub type MessageQueue = pallet_message_queue;
    #[runtime::pallet_index(10)]
    pub type Utility = pallet_utility;
    #[runtime::pallet_index(11)]
    pub type WeteeOrg = wetee_org;
    #[runtime::pallet_index(12)]
    pub type WeteeAsset = wetee_assets;
    #[runtime::pallet_index(13)]
    pub type WeteeSudo = wetee_sudo;
    #[runtime::pallet_index(14)]
    pub type WeteeGuild = wetee_guild;
    #[runtime::pallet_index(15)]
    pub type WeteeProject = wetee_project;
    #[runtime::pallet_index(16)]
    pub type WeteeGov = wetee_gov;
    #[runtime::pallet_index(17)]
    pub type WeteeTreasury = wetee_treasury;
    #[runtime::pallet_index(18)]
    pub type WeteeApp = wetee_app;
    #[runtime::pallet_index(19)]
    pub type WeteeTask = wetee_task;
    #[runtime::pallet_index(20)]
    pub type WeteeGpu = wetee_gpu;
    #[runtime::pallet_index(21)]
    pub type WeteeWorker = wetee_worker;
    // WETEE end

    // WETEE
		Tokens: orml_tokens = 107,
		RandomnessCollectiveFlip: pallet_insecure_randomness_collective_flip = 108,
		MessageQueue: pallet_message_queue = 109,
		Utility : pallet_utility = 110,
		WeteeOrg : wetee_org = 111,
		WeteeAsset : wetee_assets = 112,
		WeteeSudo : wetee_sudo = 113,
		WeteeGuild : wetee_guild = 114,
		WeteeProject : wetee_project = 115,
		WeteeGov : wetee_gov = 116,
		WeteeTreasury : wetee_treasury = 117,
		WeteeApp : wetee_app = 118,
		WeteeTask : wetee_task = 119,
		WeteeGpu : wetee_gpu = 120,
		WeteeWorker : wetee_worker = 121,
    Contracts: pallet_contracts = 122,
    // WETEE end
```

```rust
[wetee_org, WeteeOrg]
// [wetee_assets, WeteeAsset]
[wetee_sudo, WeteeSudo]
[wetee_guild, WeteeGuild]
// [wetee_project, WeteeProject]
[wetee_gov, WeteeGov]
[wetee_treasury, WeteeTreasury]

```
### build genesis
./target/release/parachain-node export-genesis-state genesis --chain wetee-rococo
### build wasm
./target/release/parachain-node export-genesis-wasm genesis-wasm --chain wetee-rococo
### build spec
./target/release/parachain-node build-spec --disable-default-bootnode --chain wetee-rococo > wetee-rococo.json

### 平行链启动命令
./target/release/parachain-node --collator \
--alice \
--chain ./wetee-rococo.json \
--force-authoring \
--base-path  ./db \
-- \
--chain=rococo \
--sync fast-unsafe \
--blocks-pruning 256 \
--state-pruning 256