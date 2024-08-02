use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
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
/// App状态 0: created, 1: deploying, 2: stop, 3: deoloyed
pub type WorkStatus = u8;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub enum WorkType {
    #[default]
    /// APP
    APP = 0,
    /// TASK
    TASK,
    /// GPU
    GPU,
}

/// WorkId
/// 工作ID
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
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
    pub disk: Vec<Disk>,
    pub gpu: u32,
}

/// 网络设置
/// disk setting
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum Service {
    /// TCP
    Tcp(u16),
    /// UDP
    Udp(u16),
    /// TCP
    Https(u16),
    /// Project Tcp
    ProjectTcp(u16),
    /// Project Udp
    ProjectUdp(u16),
}

impl Default for Service {
    fn default() -> Self {
        Service::Tcp(80) // 默认为TCP协议，端口为0
    }
}

/// 储存类型
/// disk setting
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum DiskClass {
    /// TCP
    SSD(Vec<u8>),
}

impl Default for DiskClass {
    fn default() -> Self {
        DiskClass::SSD("".as_bytes().to_vec()) // 默认为TCP协议，端口为0
    }
}

/// 储存设置
/// disk setting
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Disk {
    /// key
    pub path: DiskClass,
    /// value
    pub size: u32,
}

/// 计算资源
/// computing resource
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ComCr {
    pub cpu: u32,
    pub mem: u32,
    pub cvm_cpu: u32,
    pub cvm_mem: u32,
    pub disk: u32,
    pub gpu: u32,
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

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum EnvKey {
    /// Env 环境变量
    Env(Vec<u8>),
    /// UPDATE
    File(Vec<u8>),
}

impl Default for EnvKey {
    fn default() -> Self {
        EnvKey::Env("".as_bytes().to_vec()) // 默认为TCP协议，端口为0
    }
}

/// App setting
/// 应用设置
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Env {
    /// container index
    pub index: u16,
    /// key
    pub k: EnvKey,
    /// value
    pub v: Vec<u8>,
}

/// App setting
/// 应用设置
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct EnvInput {
    /// edit type
    pub etype: EditType,
    /// container index
    pub index: u16,
    /// key
    pub k: EnvKey,
    /// value
    pub v: Vec<u8>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum Command {
    /// /bin/sh 启动
    SH(Vec<u8>),
    /// /bin/bash 启动
    BASH(Vec<u8>),
    /// /bin/zsh 启动
    ZSH(Vec<u8>),
    NONE,
}

impl Default for Command {
    fn default() -> Self {
        Command::SH("".as_bytes().to_vec()) // 默认为TCP协议，端口为0
    }
}

/// TEEVersion
/// TEE 实现版本
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum TEEVersion {
    #[default]
    SGX,
    CVM,
}

pub type GPUtype = u16;

/// App specific information
/// 程序信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Container {
    /// img of the App.
    /// image 目标宗旨
    pub image: Vec<u8>,
    /// command of service
    /// 执行命令
    pub command: Command,
    /// port of service
    /// 服务端口号
    pub port: Vec<Service>,
    /// cpu memory disk
    /// cpu memory disk
    pub cr: Cr,
}

/// Ip 信息
/// Ip
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Ip {
    pub ipv4: Option<u32>,
    pub ipv6: Option<u128>,
    pub domain: Option<Vec<u8>>,
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct P2PAddr<AccountId> {
    /// ip of the p2p
    pub ip: Ip,
    /// port of the p2p
    pub port: u16,
    /// p2p id
    pub id: AccountId,
}
