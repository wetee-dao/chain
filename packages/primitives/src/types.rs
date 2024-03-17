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

/// Level
/// 等级
pub type ClusterLevel = u8;

/// status
/// 状态
pub type WorkStatus = u8;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum WorkType {
    #[default]
    /// APP
    APP,
    /// TASK
    TASK,
    /// GPU
    GPU,
}

/// WorkId
/// 工作ID
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct WorkId {
    pub wtype: WorkType,
    pub id: TeeAppId,
}

/// MintId
/// 挖矿ID
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct MintId {
    pub wtype: WorkType,
    pub cid: ClusterId,
    pub id: TeeAppId,
}

/// 计算资源
/// computing resource
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Cr {
    pub cpu: u32,
    pub mem: u32,
    pub disk: u32,
    // pub gpu: Option<u32>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum EditType {
    #[default]
    /// INSERT
    INSERT,
    /// UPDATE
    UPDATE(u16),
    /// REMOVE
    REMOVE(u16),
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
    /// edit type
    pub etype: EditType,
    /// key
    pub k: Vec<u8>,
    /// value
    pub v: Vec<u8>,
}
