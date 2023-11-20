pub use frame_support::codec::{Decode, Encode};

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

/// WorkerMsgId (type 1 =》app / 2=》 task )
/// 任务ID (类型 1 =》app / 2=》 task )
/// (类型,任务id) (type,id)
#[derive(Encode, Decode)]
pub struct WorkerMsgId {
    pub t: u8,
    pub id: u64,
}
