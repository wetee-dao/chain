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

use wetee_primitives::{traits::AfterCreate, types::WorkerId, vec2bytes};

use crate::sp_api_hidden_includes_construct_runtime::hidden_include::traits::EnqueueMessage;
use crate::{AccountId, MessageQueue, WeteeWorker};
use pallet_message_queue::OnQueueChanged;

/// Mocked message origin for testing.
#[derive(Clone, Eq, PartialEq, Encode, Decode, MaxEncodedLen, TypeInfo, Debug)]
pub enum MessageOrigin {
    /// 用户发起的任务
    Account(AccountId),
}

pub struct WorkerQueueHook;
impl AfterCreate<WorkerId, AccountId> for WorkerQueueHook {
    fn run_hook(id: WorkerId, aid: AccountId) {
        // 添加消息到队列
        MessageQueue::enqueue_message(vec2bytes(&id.encode()), MessageOrigin::Account(aid));
    }
}

/// 消息队列变化处理器
pub struct WorkerQueueChangeHandler;
impl OnQueueChanged<MessageOrigin> for WorkerQueueChangeHandler {
    fn on_queue_changed(_id: MessageOrigin, _items_count: u64, _items_size: u64) {
        log::warn!("on_queue_changedon_queue_changedon_queue_changedon_queue_changedon_queue_changedon_queue_changed");
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
        let msg_id = WorkerId::decode(&mut message).unwrap();
        let _required = Weight::from_parts(1, 1);
        log::warn!(
            "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ {:?}",
            id
        );
        match origin {
            MessageOrigin::Account(ref account) => {
                let _ = WeteeWorker::match_app_deploy(account.clone(), msg_id);
            }
        };

        Ok(true)
    }
}

/// 暂停的任务
pub struct WorkerQueuePauser;
impl QueuePausedQuery<MessageOrigin> for WorkerQueuePauser {
    fn is_paused(_id: &MessageOrigin) -> bool {
        // PausedQueues::get().contains(id)
        return false;
    }
}
