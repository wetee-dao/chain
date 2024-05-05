use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::traits::QueuePausedQuery;
// pub use frame_support::weights::{
//     constants::{
//         BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
//     },
//     IdentityFee, Weight,
// };
use frame_support::{
    traits::{ProcessMessage, ProcessMessageError, QueueFootprint},
    weights::WeightMeter,
};
use scale_info::TypeInfo;

use wetee_primitives::{
    traits::{UHook, WorkExt},
    types::{TEEVersion, WorkId, WorkType},
    vec2bytes,
};

use crate::{
    sp_api_hidden_includes_construct_runtime::hidden_include::traits::EnqueueMessage, Balance,
    Runtime,
};
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
impl UHook<WorkId, AccountId> for WorkerQueueHook {
    fn run_hook(id: WorkId, _: AccountId) {
        // 添加消息到队列
        MessageQueue::enqueue_message(vec2bytes(&id.encode()), MessageOrigin::Work);
    }
}

/// 消息队列变化处理器
pub struct WorkerQueueChangeHandler;
impl OnQueueChanged<MessageOrigin> for WorkerQueueChangeHandler {
    fn on_queue_changed(id: MessageOrigin, _: QueueFootprint) {
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
            MessageOrigin::Work => match msg_id.wtype {
                WorkType::APP => WeteeWorker::match_deploy(msg_id, None).unwrap(),
                WorkType::TASK => WeteeWorker::match_deploy(msg_id, None).unwrap(),
                WorkType::GPU => WeteeWorker::match_deploy(msg_id, None).unwrap(),
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

pub struct WorkExtIns;
impl WorkExt<AccountId, Balance> for WorkExtIns {
    fn work_info(
        work: WorkId,
    ) -> core::result::Result<
        (AccountId, wetee_primitives::types::Cr, u8, u8, TEEVersion),
        sp_runtime::DispatchError,
    > {
        match work.wtype {
            WorkType::APP => {
                let account = wetee_app::AppIdAccounts::<Runtime>::get(work.id)
                    .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;
                let app = wetee_app::TEEApps::<Runtime>::get(account.clone(), work.clone().id)
                    .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;
                return Ok((
                    account,
                    app.cr.clone(),
                    app.level,
                    app.status,
                    app.tee_version,
                ));
            }
            WorkType::TASK => {
                let account = wetee_task::TaskIdAccounts::<Runtime>::get(work.id)
                    .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;
                let task = wetee_task::TEETasks::<Runtime>::get(account.clone(), work.clone().id)
                    .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;
                return Ok((
                    account,
                    task.cr.clone(),
                    task.level,
                    task.status,
                    task.tee_version,
                ));
            }
            WorkType::GPU => {
                let account = wetee_gpu::AppIdAccounts::<Runtime>::get(work.id)
                    .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;
                let app = wetee_gpu::GPUApps::<Runtime>::get(account.clone(), work.clone().id)
                    .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;
                return Ok((
                    account,
                    app.cr.clone(),
                    app.level,
                    app.status,
                    app.tee_version,
                ));
            }
        }
    }

    fn set_work_status(
        work: WorkId,
        status: u8,
    ) -> core::result::Result<bool, sp_runtime::DispatchError> {
        match work.wtype {
            WorkType::APP => {
                let account = wetee_app::AppIdAccounts::<Runtime>::get(work.id)
                    .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;
                let mut app = wetee_app::TEEApps::<Runtime>::get(account.clone(), work.clone().id)
                    .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;

                app.status = status;
                wetee_app::TEEApps::<Runtime>::insert(account.clone(), work.id.clone(), app);

                return Ok(true);
            }
            WorkType::TASK => {
                let account = wetee_task::TaskIdAccounts::<Runtime>::get(work.id)
                    .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;
                let mut task =
                    wetee_task::TEETasks::<Runtime>::get(account.clone(), work.clone().id)
                        .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;

                task.status = status;
                wetee_task::TEETasks::<Runtime>::insert(account.clone(), work.id.clone(), task);

                return Ok(true);
            }
            WorkType::GPU => {
                let account = wetee_gpu::AppIdAccounts::<Runtime>::get(work.id)
                    .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;
                let mut app = wetee_gpu::GPUApps::<Runtime>::get(account.clone(), work.clone().id)
                    .ok_or(wetee_worker::Error::<Runtime>::AppNotExists)?;

                app.status = status;
                wetee_gpu::GPUApps::<Runtime>::insert(account.clone(), work.id.clone(), app);

                return Ok(true);
            }
        }
    }

    fn calculate_fee(work: WorkId) -> core::result::Result<Balance, sp_runtime::DispatchError> {
        match work.wtype {
            WorkType::APP => {
                return wetee_app::Pallet::<Runtime>::get_fee(work.id.clone());
            }
            WorkType::TASK => {
                return wetee_task::Pallet::<Runtime>::get_fee(work.id.clone());
            }
            WorkType::GPU => {
                return wetee_gpu::Pallet::<Runtime>::get_fee(work.id.clone());
            }
        }
    }

    fn pay_run_fee(
        work: WorkId,
        to: AccountId,
        fee: Balance,
    ) -> core::result::Result<u8, sp_runtime::DispatchError> {
        match work.wtype {
            WorkType::APP => {
                return wetee_app::Pallet::<Runtime>::pay_run_fee(work.clone(), fee, to);
            }
            WorkType::TASK => {
                return wetee_task::Pallet::<Runtime>::pay_run_fee(work.clone(), fee, to);
            }
            WorkType::GPU => {
                return wetee_gpu::Pallet::<Runtime>::pay_run_fee(work.clone(), fee, to);
            }
        }
    }

    fn try_stop(
        account: AccountId,
        work: WorkId,
    ) -> core::result::Result<bool, sp_runtime::DispatchError> {
        match work.wtype {
            WorkType::APP => {
                let _ = wetee_app::Pallet::<Runtime>::try_stop(account, work.id.clone())?;
                return Ok(true);
            }
            WorkType::TASK => {
                let _ = wetee_task::Pallet::<Runtime>::try_stop(account, work.id.clone())?;
                return Ok(true);
            }
            WorkType::GPU => {
                let _ = wetee_gpu::Pallet::<Runtime>::try_stop(account, work.id.clone())?;
                return Ok(true);
            }
        }
    }
}
