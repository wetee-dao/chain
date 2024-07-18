use parity_scale_codec::{Decode, Encode};
use scale_info::{prelude::vec::Vec, TypeInfo};
use sp_runtime::RuntimeDebug;

use wetee_primitives::types::{ClusterId, ComCr, WorkId};

/// K8sCluster specific information
/// 集群信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct K8sCluster<BlockNumber> {
    /// 节点id
    /// 节点id
    pub id: ClusterId,
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
    /// ip of service
    /// 服务端口号
    pub ip: Vec<Ip>,
    /// port of service
    /// 服务端口号
    pub port: u32,
    /// State of the App
    /// K8sCluster 状态
    pub status: u8,
}

/// 质押数据
/// deposit of computing resource
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Deposit<Balance> {
    /// Deposit amount
    /// 质押金额
    pub deposit: Balance,
    /// cpu
    /// cpu
    pub cpu: u32,
    pub cvm_cpu: u32,
    /// memory
    /// memory
    pub mem: u32,
    pub cvm_mem: u32,
    /// disk
    /// disk
    pub disk: u32,
    /// gpu
    /// gpu
    pub gpu: u32,
}

/// 集群证明
/// proof of K8sCluster
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ProofOfCluster {
    /// tee public key
    pub public_key: Vec<u8>,
}

/// 工作证明
/// proof of K8sCluster
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ProofOfWork {
    /// Task log address and hash
    /// 任务日志地址及hash
    pub log_hash: Vec<u8>,
    /// task cpu memory usage
    /// 任务cpu 内存 占用
    pub cr: ComCr,
    /// task cpu memory usage hash
    /// 任务cpu 内存 占用监控hash
    pub cr_hash: Vec<u8>,
}

/// 合约日志
/// Log of contract
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ContractState<BlockNumber, Balance> {
    /// block_number
    /// 区块号
    pub block_number: BlockNumber,
    /// state
    /// 状态
    pub minted: Balance,
    /// withdrawal
    /// 取回
    pub withdrawal: Balance,
}

/// 合同缓存
/// Log of contract
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct ClusterContractState<BlockNumber, AccountId> {
    /// start_number
    /// 开始区块号
    pub start_number: BlockNumber,
    /// user
    /// 用户
    pub user: AccountId,
    /// work_id
    /// 工作id
    pub work_id: WorkId,
}

/// 抵押
/// DepositPrice
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct DepositPrice {
    /// cpu
    pub cpu_per: u32,
    /// cpu
    pub cvm_cpu_per: u32,
    /// memory
    pub memory_per: u32,
    /// cvm_memory
    pub cvm_memory_per: u32,
    /// disk
    pub disk_per: u32,
    /// gpu
    pub gpu_per: u32,
}

/// Ip 信息
/// Ip
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Ip {
    pub ipv4: Option<u32>,
    pub ipv6: Option<u128>,
    pub domain: Option<Vec<u8>>,
}
