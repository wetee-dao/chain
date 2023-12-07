use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

/// Simple index type for proposal counting.
pub type ProposalIndex = u32;

/// 用户容量
pub type MemberCount = u32;

/// 真实请求ID
pub type RealCallId = u32;

/// WETEE函数index
pub type CallId = u32;

/// 资源ID
pub type DaoAssetId = u64;

/// ProjectId
/// 项目ID
pub type ProjectId = u64;

/// GuildId
/// 工会ID
pub type GuildId = u64;

/// BoardId
/// 看板ID
pub type BoardId = u64;

/// TaskId
/// 任务ID
pub type TaskId = u64;

/// TeeAppId
/// 应用ID
pub type TeeAppId = u64;

/// WorkerId (type 1 =》app / 2=》 task )
/// 任务ID (类型 1 =》app / 2=》 task )
/// (类型,任务id) (type,id)
#[derive(Default, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct WorkerId {
    pub t: u8,
    pub id: TeeAppId,
}

/// 计算资源
/// computing resource
#[derive(PartialEq, Eq, Default, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Cr {
    pub cpu: u16,
    pub memory: u16,
    pub disk: u16,
}
