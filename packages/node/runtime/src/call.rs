use crate::{GovFunc, RuntimeCall};
use wetee_primitives::traits::PalletGet;
use wetee_primitives::types::CallId;

/// 定义那些函数能被当作 sudo/gov 方式调用
impl TryFrom<RuntimeCall> for CallId {
    type Error = ();
    fn try_from(call: RuntimeCall) -> Result<Self, Self::Error> {
        call2id(call)
    }
}

impl PalletGet<RuntimeCall> for GovFunc {
    fn get_pallet_id(call: RuntimeCall) -> u16 {
        let id = call2id(call).unwrap_or_default();
        let p = (id - id % 100) / 100;
        p.try_into().unwrap_or_default()
    }
}

fn call2id(call: RuntimeCall) -> Result<u32, ()> {
    match call {
        RuntimeCall::WeteeOrg(wetee_org::Call::create_dao { .. }) => Ok(101 as CallId),
        RuntimeCall::WeteeAsset(func) => match func {
            wetee_assets::Call::create_asset { .. } => Ok(201 as CallId),
            wetee_assets::Call::set_existenial_deposit { .. } => Ok(202 as CallId),
            wetee_assets::Call::set_metadata { .. } => Ok(203 as CallId),
            wetee_assets::Call::burn { .. } => Ok(204 as CallId),
            wetee_assets::Call::transfer { .. } => Ok(205 as CallId),
            wetee_assets::Call::join { .. } => Ok(206 as CallId),
            _ => Err(()),
        },
        RuntimeCall::WeteeGuild(func) => match func {
            wetee_guild::Call::guild_join { .. } => Ok(301 as CallId),
            wetee_guild::Call::create_guild { .. } => Ok(302 as CallId),
            _ => Err(()),
        },
        RuntimeCall::WeteeGov(func) => match func {
            wetee_gov::Call::submit_proposal { .. } => Ok(401 as CallId),
            // wetee_gov::Call::recreate { .. } => Ok(402 as CallId),
            wetee_gov::Call::deposit_proposal { .. } => Ok(403 as CallId),
            wetee_gov::Call::vote_for_prop { .. } => Ok(404 as CallId),
            wetee_gov::Call::cancel_vote { .. } => Ok(405 as CallId),
            wetee_gov::Call::run_proposal { .. } => Ok(406 as CallId),
            wetee_gov::Call::unlock { .. } => Ok(407 as CallId),
            wetee_gov::Call::set_max_pre_props { .. } => Ok(409 as CallId),
            wetee_gov::Call::update_vote_model { .. } => Ok(415 as CallId),
            _ => Err(()),
        },
        RuntimeCall::WeteeProject(func) => match func {
            wetee_project::Call::project_join_request { .. } => Ok(501 as CallId),
            wetee_project::Call::create_project { .. } => Ok(502 as CallId),
            wetee_project::Call::apply_project_funds { .. } => Ok(503 as CallId),
            wetee_project::Call::create_task { .. } => Ok(504 as CallId),
            wetee_project::Call::join_task { .. } => Ok(505 as CallId),
            wetee_project::Call::leave_task { .. } => Ok(506 as CallId),
            wetee_project::Call::join_task_review { .. } => Ok(507 as CallId),
            wetee_project::Call::leave_task_review { .. } => Ok(508 as CallId),
            wetee_project::Call::start_task { .. } => Ok(509 as CallId),
            wetee_project::Call::request_review { .. } => Ok(510 as CallId),
            wetee_project::Call::task_done { .. } => Ok(511 as CallId),
            wetee_project::Call::make_review { .. } => Ok(512 as CallId),
            _ => Err(()),
        },
        RuntimeCall::WeteeTreasury(func) => match func {
            wetee_treasury::Call::spend { .. } => Ok(601 as CallId),
            _ => Err(()),
        },
        _ => Err(()),
    }
}
