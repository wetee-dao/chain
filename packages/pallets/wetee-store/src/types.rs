use parity_scale_codec::{Decode, Encode};
use scale_info::{prelude::vec::Vec, TypeInfo};
use sp_runtime::RuntimeDebug;

/// app template
/// 应用模版
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct AppTemplate<AccountId, BlockNumber> {
    /// 节点id
    /// 节点id
    pub id: u128,
    /// creator of K8sCluster
    /// 创建者
    pub account: AccountId,
    /// The block that creates the K8sCluster
    /// App创建的区块
    pub start_block: BlockNumber,
    /// Stop time
    /// 停止时间
    pub stop_block: Option<BlockNumber>,
    /// terminal time
    /// 终止时间
    pub terminal_block: Option<BlockNumber>,
    /// name of the K8sCluster.
    /// 集群名字
    pub name: Vec<u8>,
    /// app type
    /// 程序运行的类型
    pub ty: AppType,
    /// runtime type
    /// TEE 运行类型
    pub run: RuntimeType,
    /// State of the App
    /// K8sCluster 状态
    pub status: u8,
}

/// config of container
/// 容器配置
#[derive(Default, PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Image {
    /// 容器镜像
    /// image
    pub i: Vec<u8>,
    /// 容器端口
    /// port
    pub p: Vec<Port>,
    /// 环境变量
    /// env
    pub e: Vec<Env>,
    /// 磁盘
    /// disk
    pub v: Vec<Volume>,
    /// 容器命令
    /// pre command
    pub pc: Vec<u8>,
    /// 容器命令
    /// command
    pub c: Vec<u8>,
}

/// 容器端口
/// port
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Port {
    pub k: Vec<u8>,
    pub v: u16,
}

/// 环境变量
/// environment variable
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Env {
    pub k: Vec<u8>,
    pub v: Vec<u8>,
}

/// 容器
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct Volume {
    pub k: Vec<u8>,
    pub v: u32,
}

/// App run type
/// 程序运行的类型
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum AppType {
    /// Service
    Service,
    /// Task
    Task,
    /// Ai
    Ai,
}

/// Runtime type
/// TEE 运行类型
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum RuntimeType {
    /// SGX
    SGX,
    /// CVM
    CVM,
}
