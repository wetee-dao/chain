use parity_scale_codec::{Decode, Encode, MaxEncodedLen};
use scale_info::prelude::vec;
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

/// 字符串
/// String
pub type TeeString = Vec<u8>;

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

impl Default for Disk {
    fn default() -> Self {
        Disk {
            path: DiskClass::SSD("".as_bytes().to_vec()),
            size: 1,
        }
    }
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

impl Default for EnvInput {
    fn default() -> Self {
        EnvInput {
            etype: EditType::INSERT,
            index: 1,
            k: EnvKey::Env(vec![0; 16]),
            v: vec![0; 16],
        }
    }
}

/// 加密配置 hash
/// secret setting hash
pub type EnvHash = Vec<u8>;

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

impl Default for Container {
    fn default() -> Self {
        Container {
            image: "".as_bytes().to_vec(),
            command: Command::NONE,
            port: vec![Service::default()],
            cr: Cr::default(),
        }
    }
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

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct ApiMeta {
    pub port: u16,
    pub apis: Vec<Api>,
}

#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Api {
    // url
    pub url: Vec<u8>,
    // 0: get, 1: post, 2: put, 3: delete
    pub method: u8,
}

#[derive(Encode, Decode, Clone, Debug, TypeInfo, PartialEq)]
pub enum InkArg {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    U128(u128),
    I128(i128),
    TString(Vec<u8>),
}

impl InkArg {
    pub fn encode2vec(&self) -> Vec<u8> {
        match self {
            InkArg::Bool(b) => b.encode(),
            InkArg::U8(u) => u.encode(),
            InkArg::I8(i) => i.encode(),
            InkArg::U16(u) => u.encode(),
            InkArg::I16(i) => i.encode(),
            InkArg::U32(u) => u.encode(),
            InkArg::I32(i) => i.encode(),
            InkArg::U64(u) => u.encode(),
            InkArg::I64(i) => i.encode(),
            InkArg::U128(u) => u.encode(),
            InkArg::I128(i) => i.encode(),
            InkArg::TString(s) => s.encode(),
        }
    }
}
