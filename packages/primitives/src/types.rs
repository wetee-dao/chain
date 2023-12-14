use codec::{Decode, Encode};
use scale_info::{prelude::vec::Vec, TypeInfo};
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

/// TeeAppId
/// 应用ID
pub type ClusterId = u64;

/// WorkId (type 1 =》app / 2=》 task )
/// 任务ID (类型 1 =》app / 2=》 task )
/// (类型,任务id) (type,id)
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct WorkId {
    // 1 =》app
    // 2=》 task
    pub t: u8,
    pub id: TeeAppId,
}

/// WorkId (type 1 =》app / 2=》 task )
/// 任务ID (类型 1 =》app / 2=》 task )
/// (类型,任务id) (type,id)
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct MintId {
    // 1 =》app
    // 2=》 task
    pub t: u8,
    pub cid: ClusterId,
    pub id: TeeAppId,
}

/// 计算资源
/// computing resource
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Cr {
    pub cpu: u16,
    pub memory: u16,
    pub disk: u16,
}

/// App setting
/// 应用设置
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct AppSetting {
    /// key
    pub k: Vec<u8>,
    /// value
    pub v: Vec<u8>,
}

/// App setting
/// 应用设置
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct AppSettingInput {
    /// 1: insert, 1: update, 3: remove
    pub t: u8,
    /// index of the setting
    pub index: u16,
    /// key
    pub k: Vec<u8>,
    /// value
    pub v: Vec<u8>,
}
