use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::QueuePausedQuery;
pub use frame_support::weights::{
    constants::{
        BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
    },
    IdentityFee, Weight,
};
use frame_support::{
    traits::{ProcessMessage, ProcessMessageError},
    weights::WeightMeter,
};
use scale_info::TypeInfo;

use wetee_primitives::{traits::AfterCreate, types::WorkId, vec2bytes};

use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::EnqueueMessage;
use crate::{AccountId, MessageQueue, WeteeWorker};
use pallet_message_queue::OnQueueChanged;

/// Mocked message origin for testing.
/// 消息来源
#[derive(Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
pub enum MessageOrigin {
    /// 用户发起的任务
    Work,
}

/// 任务队列变化处理器
pub struct WorkerQueueHook;
impl AfterCreate<WorkId, AccountId> for WorkerQueueHook {
    fn run_hook(id: WorkId, _: AccountId) {
        // 添加消息到队列
        MessageQueue::enqueue_message(vec2bytes(&id.encode()), MessageOrigin::Work);
    }
}

/// 消息队列变化处理器
pub struct WorkerQueueChangeHandler;
impl OnQueueChanged<MessageOrigin> for WorkerQueueChangeHandler {
    fn on_queue_changed(id: MessageOrigin, _items_count: u64, _items_size: u64) {
        log::info!("on_queue_changed ===> {:?}", id);
        // QueueChanges::mutate(|cs| cs.push((id, items_count, items_size)));
    }
}

/// 消息处理器
pub struct WorkerMessageProcessor;
impl ProcessMessage for WorkerMessageProcessor {
    type Origin = MessageOrigin;

    /// Process a message.
    /// 处理消息
    fn process_message(
        mut message: &[u8],
        origin: Self::Origin,
        _meter: &mut WeightMeter,
        id: &mut [u8; 32],
    ) -> Result<bool, ProcessMessageError> {
        let msg_id = WorkId::decode(&mut message).unwrap();
        log::warn!("process_message {:?}", id);
        let ok: bool = match origin {
            MessageOrigin::Work => match msg_id.t {
                1 => WeteeWorker::match_app_deploy(msg_id, None).unwrap(),
                2 => WeteeWorker::match_task_deploy(msg_id, None).unwrap(),
                _ => false,
            },
        };

        if !ok {
            return Err(ProcessMessageError::Yield);
        };

        Ok(ok)
    }
}

/// 暂停的任务
pub struct WorkerQueuePauser;
impl QueuePausedQuery<MessageOrigin> for WorkerQueuePauser {
    /// Check if a queue is paused.
    fn is_paused(_id: &MessageOrigin) -> bool {
        return false;
    }
}
