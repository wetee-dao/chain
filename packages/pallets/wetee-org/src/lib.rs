#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::type_complexity)]
use codec::MaxEncodedLen;
use codec::{Decode, Encode};
use frame_support::traits::IsSubType;
pub use pallet::*;
use scale_info::TypeInfo;
use sp_runtime::{traits::BlockNumberProvider, RuntimeDebug};
use sp_std::{prelude::*, result};
use wetee_primitives::{
    traits::AfterCreate,
    types::{DaoAssetId, GuildId, ProjectId, TaskId},
};

mod weights;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

/// DAO's status.
/// 组织状态
#[derive(Default, PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub enum Status {
    #[default]
    /// In use.
    /// 激活
    Active = 0,
    /// Does not work properly.
    /// 未激活
    InActive,
}

/// DAO specific information
/// 组织信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct OrgInfo<AccountId, BlockNumber> {
    pub id: DaoAssetId,
    /// creator of DAO
    /// 创建者
    pub creator: AccountId,
    /// The block that creates the DAO
    /// DAO创建的区块
    pub start_block: BlockNumber,
    /// DAO account id.
    /// DAO 链上账户ID
    pub dao_account_id: AccountId,
    /// name of the DAO.
    /// DAO 名字
    pub name: Vec<u8>,
    /// name of the DAO.
    /// DAO 介绍
    pub desc: Vec<u8>,
    /// Purpose of the DAO.
    /// DAO 目标宗旨
    pub purpose: Vec<u8>,
    //// meta data
    /// DAO 元数据 图片等内容
    pub meta_data: Vec<u8>,
    /// im api
    pub im_api: Vec<u8>,
    /// org color
    pub bg: Vec<u8>,
    /// org logo
    pub logo: Vec<u8>,
    /// 组织大图
    pub img: Vec<u8>,
    /// 组织主页
    pub home_url: Vec<u8>,
    /// State of the DAO
    /// DAO状态
    pub status: Status,
}

/// DAO specific information
/// 组织应用信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct OrgApp<BlockNumber> {
    pub id: DaoAssetId,
    pub app_id: DaoAssetId,
    /// The block that creates the DAO
    /// DAO创建的区块
    pub start_block: BlockNumber,
    /// name of the DAO.
    /// DAO 名字
    pub name: Vec<u8>,
    /// name of the DAO.
    /// DAO 介绍
    pub desc: Vec<u8>,
    /// icon of the DAO.
    /// DAO icon
    pub icon: Vec<u8>,
    //// url data
    /// url 图片等内容
    pub url: Vec<u8>,
    /// State of the OrgApp
    /// OrgApp 状态
    pub status: Status,
}

/// DAO specific information
/// 组织应用信息
#[derive(Default, PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct App<AccountId> {
    pub id: DaoAssetId,
    /// url of the App.
    /// App url
    pub url: Vec<u8>,
    /// name of the App.
    /// App 名字
    pub name: Vec<u8>,
    /// name of the App.
    /// App 介绍
    pub desc: Vec<u8>,
    /// icon of the App.
    /// App icon
    pub icon: Vec<u8>,
    /// creator of Task
    /// 创建者
    pub creator: AccountId,
    /// State of the App
    /// App 状态
    pub status: Status,
}

/// Guild information
/// 组织内公会信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct GuildInfo<AccountId, BlockNumber> {
    /// id of GuildInfo
    pub id: GuildId,
    /// creator of DAO
    /// 创建者
    pub creator: AccountId,
    /// The block that creates the DAO
    /// DAO创建的区块
    pub start_block: BlockNumber,
    /// DAO account id.
    /// DAO 链上账户ID
    pub dao_account_id: AccountId,
    /// Purpose of the DAO.
    /// DAO 目标宗旨
    pub name: Vec<u8>,
    /// Purpose of the DAO.
    /// DAO 目标宗旨
    pub desc: Vec<u8>,
    //// meta data
    /// DAO 元数据 图片等内容
    pub meta_data: Vec<u8>,
    /// State of the DAO
    /// DAO状态
    pub status: Status,
}

/// task specific information
/// 任务信息
#[derive(PartialEq, Eq, Clone, RuntimeDebug, Encode, Decode, TypeInfo)]
pub struct QuarterTask<AccountId> {
    pub id: TaskId,
    /// 任务名称
    /// name of the Task.
    pub name: Vec<u8>,
    /// priority
    /// 优先程度
    pub priority: u8,
    /// creator of Task
    /// 创建者
    pub creator: AccountId,
    /// tag info
    /// 数据标签
    pub tags: Vec<u8>,
    /// State of the Task
    /// DAO状态
    /// ToDo = 0,
    /// InProgress = 1,
    /// InReview = 2,
    /// Done = 3,
    pub status: u8,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct DaoAssetAccount {
    pub dao_id: DaoAssetId,
    pub t: u8,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct DaoGovAccount {
    // 组织id
    pub id: DaoAssetId,
    // 投票轨道
    pub p: u32,
    // 是否同意
    pub s: u8,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct DaoProjectAccount {
    pub dao_id: DaoAssetId,
    pub project_id: ProjectId,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, Default, RuntimeDebug, TypeInfo)]
pub struct DaoGuildAccount {
    pub dao_id: DaoAssetId,
    pub guild_id: GuildId,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        dispatch::{DispatchResultWithPostInfo, GetDispatchInfo},
        pallet_prelude::*,
        traits::UnfilteredDispatchable,
        PalletId,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::AccountIdConversion;

    /// pallet config
    /// 组件配置文件
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// pallet event
        /// 组件消息
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// All calls supported by DAO
        /// 组件所有的函数
        type RuntimeCall: Parameter
            + UnfilteredDispatchable<RuntimeOrigin = Self::RuntimeOrigin>
            + GetDispatchInfo
            + From<frame_system::Call<Self>>
            + From<Call<Self>>
            + IsSubType<Call<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeCall>;

        /// Each Call has its own id
        /// 函数的调用id
        type CallId: Parameter
            + Copy
            + MaybeSerializeDeserialize
            + TypeInfo
            + MaxEncodedLen
            + Default
            + TryFrom<<Self as pallet::Config>::RuntimeCall>;

        /// pallet id
        /// 模块id
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// Do some things after creating dao, such as setting up a sudo account.
        /// 创建DAO之后的回调
        type AfterCreate: AfterCreate<Self::AccountId, DaoAssetId>;

        /// max member number
        /// 组织最大的人数
        type MaxMembers: Get<u32>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
    }

    const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

    #[pallet::pallet]
    #[pallet::storage_version(STORAGE_VERSION)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// All DAOs that have been created.
    /// 所有组织
    #[pallet::storage]
    #[pallet::getter(fn daos)]
    pub type Daos<T: Config> =
        StorageMap<_, Identity, DaoAssetId, OrgInfo<T::AccountId, BlockNumberFor<T>>>;

    #[pallet::type_value]
    pub fn DefaultForm5000() -> DaoAssetId {
        5000
    }

    /// The id of the next dao to be created.
    /// 获取下一个组织id
    #[pallet::storage]
    #[pallet::getter(fn next_dao_id)]
    pub type NextDaoId<T: Config> = StorageValue<_, DaoAssetId, ValueQuery, DefaultForm5000>;

    /// The id of the next dao to be created.
    /// 获取下一个组织id
    #[pallet::storage]
    #[pallet::getter(fn next_app_id)]
    pub type NextAppId<T: Config> = StorageValue<_, DaoAssetId, ValueQuery>;

    /// the info of grutypes
    /// 组织内公会信息
    #[pallet::storage]
    #[pallet::getter(fn guilds)]
    pub type Guilds<T: Config> = StorageMap<
        _,
        Twox64Concat,
        DaoAssetId,
        BoundedVec<GuildInfo<T::AccountId, BlockNumberFor<T>>, ConstU32<100>>,
        ValueQuery,
    >;

    /// the roadmap info of projects
    /// 组织内Roadmap信息
    #[pallet::storage]
    #[pallet::getter(fn road_maps)]
    pub type RoadMaps<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        DaoAssetId,
        Twox64Concat,
        u32,
        BoundedVec<QuarterTask<T::AccountId>, ConstU32<100>>,
        ValueQuery,
    >;

    /// The id of the next dao to be created.
    /// 获取下一个组织id
    #[pallet::storage]
    #[pallet::getter(fn next_task_id)]
    pub type NextTaskId<T: Config> = StorageValue<_, ProjectId, ValueQuery>;

    /// team members
    /// 团队的成员
    #[pallet::storage]
    #[pallet::getter(fn members)]
    pub type Members<T: Config> = StorageMap<
        _,
        Twox64Concat,
        DaoAssetId,
        BoundedVec<T::AccountId, T::MaxMembers>,
        ValueQuery,
    >;

    /// guild members
    /// 公会成员
    #[pallet::storage]
    #[pallet::getter(fn guild_members)]
    pub type GuildMembers<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        DaoAssetId,
        Twox64Concat,
        u64,
        BoundedVec<T::AccountId, T::MaxMembers>,
        ValueQuery,
    >;

    /// project members
    /// 项目成员
    #[pallet::storage]
    #[pallet::getter(fn project_members)]
    pub type ProjectMembers<T: Config> = StorageDoubleMap<
        _,
        Twox64Concat,
        DaoAssetId,
        Twox64Concat,
        ProjectId,
        BoundedVec<T::AccountId, T::MaxMembers>,
        ValueQuery,
    >;

    /// apps hubs
    /// 应用中心
    #[pallet::storage]
    #[pallet::getter(fn app_hubs)]
    pub type AppHubs<T: Config> =
        StorageMap<_, Twox64Concat, DaoAssetId, App<T::AccountId>, OptionQuery>;

    /// org apps
    /// 应用中心
    #[pallet::storage]
    #[pallet::getter(fn org_apps)]
    pub type OrgApps<T: Config> = StorageMap<
        _,
        Twox64Concat,
        DaoAssetId,
        BoundedVec<OrgApp<BlockNumberFor<T>>, ConstU32<100>>,
        ValueQuery,
    >;

    /// point
    /// 成员贡献点
    #[pallet::storage]
    #[pallet::getter(fn member_point)]
    pub type MemberPoint<T: Config> =
        StorageDoubleMap<_, Twox64Concat, DaoAssetId, Twox64Concat, T::AccountId, u32, ValueQuery>;

    /// success event
    /// 成功事件
    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// DAO create event
        /// DAO创建成功事件
        CreatedDao(T::AccountId, DaoAssetId),
        /// nomal success
        /// 成功的事件
        Success,
        /// task create event
        /// 任务创建成功事件
        TaskCreated(DaoAssetId, u32, u64, T::AccountId),
        /// task update event
        /// 任务更新成功事件
        TaskUpdated(DaoAssetId, u32, u64, T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Do not have permission to create.
        /// 没有创建的权限
        HaveNoCreatePermission,
        /// DAO already exists
        /// 组织已存在
        DaoExists,
        /// DAO does not exist.
        /// 组织不存在
        DaoNotExists,
        /// guild create error
        /// 公会创建失败
        GuildCreateError,
        /// guild does not exist.
        /// 公会不存在
        GuildNotExists,
        /// DAO unsupported call
        /// 无效的调用
        InVailCall,
        /// Wrong origin.
        /// 错误的组织
        BadOrigin,
        /// Not the id of this dao.
        /// 组织 id 不正确
        DaoIdNotMatch,
        /// The description of the DAO is too long.
        /// 名字太长
        NameTooLong,
        /// The description of the DAO is too long.
        /// 组织介绍太长
        DescTooLong,
        /// The description of the DAO is too long.
        /// 组织目标太长
        PurposeTooLong,
        /// The description of the DAO is too long.
        /// 组织目标太长
        MetaDataTooLong,
        /// Numerical calculation overflow error.
        /// 溢出错误
        Overflow,
        /// member number is too long
        /// 成员数量太大
        TooManyMembers,
        /// Wrong dao gov origin.
        /// 错误的dao组织账户
        BadDaoGovOrigin,
        /// Wrong dao gov origin.
        /// 错误的dao组织账户
        BadGovOrigin,
        /// Wrong dao gov 403.
        /// 错误的dao组织账户
        BadDaoGov403,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a DAO
        /// 从一个通证池,创建一个组织
        #[pallet::call_index(001)]
        #[pallet::weight(T::WeightInfo::create_dao())]
        pub fn create_dao(
            origin: OriginFor<T>,
            name: Vec<u8>,
            desc: Vec<u8>,
            purpose: Vec<u8>,
            meta_data: Vec<u8>,
            im_api: Vec<u8>,
            bg: Vec<u8>,
            logo: Vec<u8>,
            img: Vec<u8>,
            home_url: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            ensure!(name.len() <= 30, Error::<T>::NameTooLong);
            ensure!(desc.len() <= 50, Error::<T>::DescTooLong);
            ensure!(purpose.len() <= 50, Error::<T>::PurposeTooLong);
            ensure!(meta_data.len() <= 1024, Error::<T>::MetaDataTooLong);

            let creator = ensure_signed(origin)?;

            // 创建 DAO
            let dao_id = NextDaoId::<T>::get();
            let now = frame_system::Pallet::<T>::current_block_number();
            Daos::<T>::insert(
                dao_id.clone(),
                OrgInfo {
                    id: dao_id.clone(),
                    name: name.clone(),
                    creator: creator.clone(),
                    start_block: now,
                    desc,
                    purpose,
                    status: Status::Active,
                    dao_account_id: Self::dao_account(dao_id),
                    meta_data,
                    im_api,
                    bg,
                    logo,
                    img,
                    home_url,
                },
            );

            // 初始化会员
            Self::try_add_member(dao_id, creator.clone())?;

            // 创建核心团队-coreTeam
            let mut guilds = <Guilds<T>>::get(dao_id);
            let dao_account_id = Self::dao_guild(dao_id, 0);
            guilds
                .try_insert(
                    0,
                    GuildInfo {
                        id: 0,
                        creator: creator.clone(),
                        dao_account_id,
                        start_block: now,
                        name: "CORE TEAM".as_bytes().to_vec(),
                        desc: "CORE TEAM".as_bytes().to_vec(),
                        status: Status::Active,
                        meta_data: "{}".as_bytes().to_vec(),
                    },
                )
                .map_err(|_| Error::<T>::GuildCreateError)?;

            <Guilds<T>>::insert(dao_id, &guilds);

            // 获取
            Self::try_add_guild_member(dao_id, 0, creator.clone())?;

            // 记录下一个 DAO id
            let next_id = dao_id.checked_add(1).ok_or(Error::<T>::Overflow)?;
            NextDaoId::<T>::put(next_id);

            // 执行 DAO 创建后回调
            T::AfterCreate::run_hook(creator.clone(), dao_id);

            Self::deposit_event(Event::CreatedDao(creator, dao_id));
            Ok(().into())
        }

        /// update dao info
        /// 更新组织信息
        #[pallet::call_index(008)]
        #[pallet::weight(T::WeightInfo::update_dao())]
        pub fn update_dao(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            name: Option<Vec<u8>>,
            desc: Option<Vec<u8>>,
            purpose: Option<Vec<u8>>,
            meta_data: Option<Vec<u8>>,
            im_api: Option<Vec<u8>>,
            bg: Option<Vec<u8>>,
            logo: Option<Vec<u8>>,
            img: Option<Vec<u8>>,
            home_url: Option<Vec<u8>>,
            status: Option<Status>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;

            // 只有提案的方式能更新组织信息
            let daogov = Pallet::<T>::ensrue_gov_approve_account(me)?;
            ensure!(daogov.1.id == dao_id, Error::<T>::BadDaoGovOrigin);

            // 获取组织
            let mut dao = Daos::<T>::get(dao_id).ok_or(Error::<T>::DaoNotExists)?;

            if let Some(name) = name {
                dao.name = name;
            }
            if let Some(desc) = desc {
                dao.desc = desc;
            }
            if let Some(purpose) = purpose {
                dao.purpose = purpose;
            }
            if let Some(meta_data) = meta_data {
                dao.meta_data = meta_data;
            }
            if let Some(im_api) = im_api {
                dao.im_api = im_api;
            }
            if let Some(bg) = bg {
                dao.bg = bg;
            }
            if let Some(logo) = logo {
                dao.logo = logo;
            }
            if let Some(img) = img {
                dao.img = img;
            }
            if let Some(home_url) = home_url {
                dao.home_url = home_url;
            }
            if let Some(status) = status {
                dao.status = status;
            }

            // 更新组织
            Daos::<T>::insert(dao_id, dao);

            Self::deposit_event(Event::Success);

            Ok(().into())
        }

        /// create task
        /// 创建任务
        #[pallet::call_index(002)]
        #[pallet::weight(T::WeightInfo::create_roadmap_task())]
        pub fn create_roadmap_task(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            roadmap_id: u32,
            name: Vec<u8>,
            priority: u8,
            tags: Option<Vec<u8>>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            // Pallet::<T>::ensrue_gov_approve_account(me.clone(), dao_id)?;

            // 初始化任务 id
            let task_id = NextTaskId::<T>::get();
            let mut task = QuarterTask {
                id: task_id,
                name,
                priority,
                tags: [].into(),
                creator: me.clone(),
                status: 0,
            };
            if tags.is_some() {
                task.tags = tags.unwrap();
            }

            // 插入任务id
            let mut tasks = <RoadMaps<T>>::get(dao_id, roadmap_id);
            tasks
                .try_insert(tasks.len(), task)
                .map_err(|_| Error::<T>::TooManyMembers)?;
            <RoadMaps<T>>::insert(dao_id, roadmap_id, tasks);

            // taskid 自增
            NextTaskId::<T>::put(task_id + 1);

            Self::deposit_event(Event::TaskCreated(dao_id, roadmap_id, task_id, me));

            Ok(().into())
        }

        /// update task
        /// 更新任务
        #[pallet::call_index(003)]
        #[pallet::weight(T::WeightInfo::update_roadmap_task())]
        pub fn update_roadmap_task(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            roadmap_id: u32,
            task_id: TaskId,
            priority: u8,
            status: u8,
            tags: Option<Vec<u8>>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            // Self::check_auth_for_project(dao_id, project_id, me.clone())?;

            let (mut tasks, _, index) = Self::get_task(dao_id, roadmap_id, task_id).unwrap();

            // 插入任务id
            tasks[index].priority = priority;
            tasks[index].status = status;
            if tags.is_some() {
                tasks[index].tags = tags.unwrap();
            }
            <RoadMaps<T>>::insert(dao_id, roadmap_id, tasks);

            Self::deposit_event(Event::TaskUpdated(dao_id, roadmap_id, task_id, me));

            Ok(().into())
        }

        /// create app
        /// 创建APP
        #[pallet::call_index(004)]
        #[pallet::weight(T::WeightInfo::create_app())]
        pub fn create_app(
            origin: OriginFor<T>,
            name: Vec<u8>,
            desc: Vec<u8>,
            icon: Vec<u8>,
            url: Vec<u8>,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;
            // 插入app
            let app_id = NextAppId::<T>::get();

            // 初始化app
            let app = App {
                id: app_id,
                creator: me.clone(),
                name,
                desc,
                icon,
                url,
                status: Status::Active,
            };

            <AppHubs<T>>::insert(app_id, app);
            // 记录下一个 DAO id
            let next_id = app_id.checked_add(1).ok_or(Error::<T>::Overflow)?;
            NextAppId::<T>::put(next_id);

            Self::deposit_event(Event::Success);

            Ok(().into())
        }

        /// update app status
        /// 更新APP状态
        #[pallet::call_index(005)]
        #[pallet::weight(T::WeightInfo::update_app_status())]
        pub fn update_app_status(
            origin: OriginFor<T>,
            app_id: DaoAssetId,
            status: Status,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;

            // 插入app
            let mut app = <AppHubs<T>>::get(app_id).unwrap();
            ensure!(app.creator == me, "only creator can update app status");

            app.status = status;
            <AppHubs<T>>::insert(app_id, app);

            Self::deposit_event(Event::Success);

            Ok(().into())
        }

        /// org integrate app
        /// 组织集成应用
        #[pallet::call_index(006)]
        #[pallet::weight(T::WeightInfo::org_integrate_app())]
        pub fn org_integrate_app(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            app_id: DaoAssetId,
        ) -> DispatchResultWithPostInfo {
            let _me = ensure_signed(origin)?;
            // Pallet::<T>::ensrue_gov_approve_account(me, dao_id)?;

            // 插入app
            let app = <AppHubs<T>>::get(app_id).unwrap();
            ensure!(app.status == Status::Active, "app is not active");

            let now = frame_system::Pallet::<T>::current_block_number();

            let mut apps = <OrgApps<T>>::get(dao_id);
            let org_app = OrgApp {
                id: apps.len().try_into().unwrap(),
                app_id: app.id,
                start_block: now,
                name: app.name,
                desc: app.desc,
                icon: app.icon,
                url: app.url,
                status: Status::Active,
            };

            apps.try_insert(apps.len(), org_app)
                .map_err(|_| Error::<T>::TooManyMembers)?;
            <OrgApps<T>>::insert(dao_id, apps);

            Self::deposit_event(Event::Success);

            Ok(().into())
        }

        /// 更新APP状态
        #[pallet::call_index(007)]
        #[pallet::weight(T::WeightInfo::update_org_app_status())]
        pub fn update_org_app_status(
            origin: OriginFor<T>,
            dao_id: DaoAssetId,
            app_id: DaoAssetId,
            status: Status,
        ) -> DispatchResultWithPostInfo {
            let me = ensure_signed(origin)?;

            let daogov = Pallet::<T>::ensrue_gov_approve_account(me)?;
            ensure!(daogov.1.id == dao_id, Error::<T>::BadDaoGovOrigin);

            // 插入app
            let mut apps = <OrgApps<T>>::get(dao_id);
            let (index, _app) = apps
                .iter()
                .enumerate()
                .find(|(_, app)| app.id == app_id)
                .ok_or(Error::<T>::DaoNotExists)?;

            apps[index].status = status;
            <OrgApps<T>>::insert(dao_id, apps);

            Self::deposit_event(Event::Success);

            Ok(().into())
        }
    }

    impl<T: Config> Pallet<T> {
        /// 获取DAO账户
        pub fn dao_account(dao_id: DaoAssetId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(dao_id)
        }

        /// 获取 DAO 账户
        pub fn dao_asset_pending(dao_id: DaoAssetId) -> T::AccountId {
            // pending 资金池
            T::PalletId::get().into_sub_account_truncating(DaoAssetAccount { dao_id, t: 2 })
        }

        /// 获取账户中携带的信息
        pub fn get_asset_account(x: T::AccountId) -> DaoAssetAccount {
            let (_, asset) = PalletId::try_from_sub_account::<DaoAssetAccount>(&x).unwrap();
            asset
        }

        /// 获取 DAO treasury 账户
        pub fn dao_treasury(dao_id: DaoAssetId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(DaoAssetAccount { dao_id, t: 3 })
        }

        /// 获取 DAO approve 账户
        pub fn dao_approve(dao_id: DaoAssetId, period_id: u32) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(DaoGovAccount {
                id: dao_id,
                p: period_id,
                s: 1,
            })
        }

        /// 获取 DAO reject 账户
        pub fn dao_reject(dao_id: DaoAssetId, period_id: u32) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(DaoGovAccount {
                id: dao_id,
                p: period_id,
                s: 0,
            })
        }

        /// 获取账户中携带的信息
        pub fn get_gov_account(id: T::AccountId) -> result::Result<DaoGovAccount, DispatchError> {
            let result = PalletId::try_from_sub_account::<DaoGovAccount>(&id);
            ensure!(result.is_some(), Error::<T>::BadGovOrigin);
            Ok(result.unwrap().1)
        }

        /// 确认为 DAO 决策账户
        pub fn ensrue_gov_approve_account(
            who: T::AccountId,
        ) -> result::Result<(T::AccountId, DaoGovAccount), DispatchError> {
            #[cfg(feature = "runtime-benchmarks")]
            return Ok((
                who,
                DaoGovAccount {
                    id: 5000,
                    p: 0,
                    s: 1,
                },
            ));

            // 获取账户中的信息
            let gov = Self::get_gov_account(who.clone())?;

            ensure!(gov.s == 1, Error::<T>::BadDaoGov403);
            return Ok((Self::dao_account(gov.id), gov));
        }

        /// 获取 DAO 项目 账户
        pub fn dao_project(dao_id: DaoAssetId, p_id: ProjectId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(DaoProjectAccount {
                dao_id,
                project_id: p_id,
            })
        }

        /// 获取 DAO guild 账户
        pub fn dao_guild(dao_id: DaoAssetId, p_id: ProjectId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(DaoGuildAccount {
                dao_id,
                guild_id: p_id,
            })
        }

        /// 获取创建者
        pub fn try_get_creator(dao_id: DaoAssetId) -> result::Result<T::AccountId, DispatchError> {
            let dao = Daos::<T>::get(dao_id).ok_or(Error::<T>::DaoNotExists)?;
            Ok(dao.creator)
        }

        /// 获取组织信息
        pub fn try_get_dao(
            dao_id: DaoAssetId,
        ) -> Result<OrgInfo<T::AccountId, BlockNumberFor<T>>, DispatchError> {
            let dao = Daos::<T>::get(dao_id).ok_or(Error::<T>::DaoNotExists)?;
            Ok(dao)
        }

        /// 获取公会信息
        pub fn try_get_guild(
            dao_id: DaoAssetId,
            guild_index: u32,
        ) -> Result<GuildInfo<T::AccountId, BlockNumberFor<T>>, DispatchError> {
            let guilds = <Guilds<T>>::get(dao_id);
            let guild = guilds
                .get(guild_index as usize)
                .ok_or(Error::<T>::DaoNotExists)?;
            Ok(guild.clone())
        }

        /// 获取 DAO 账户ID
        pub fn try_get_dao_account_id(
            dao_id: DaoAssetId,
        ) -> result::Result<T::AccountId, DispatchError> {
            let dao = Daos::<T>::get(dao_id).ok_or(Error::<T>::DaoNotExists)?;
            Ok(dao.dao_account_id)
        }

        /// 确认为 DAO 本账户
        pub fn ensrue_dao_root(
            who: T::AccountId,
            dao_id: DaoAssetId,
        ) -> result::Result<T::AccountId, DispatchError> {
            let dao_account_id = Self::try_get_dao_account_id(dao_id)?;
            ensure!(who == dao_account_id, Error::<T>::BadOrigin);
            Ok(who)
        }

        /// 获取创建者
        pub fn ensrue_dao_creator(
            who: T::AccountId,
            dao_id: DaoAssetId,
        ) -> result::Result<T::AccountId, DispatchError> {
            let dao = Daos::<T>::get(dao_id).ok_or(Error::<T>::DaoNotExists)?;
            ensure!(who == dao.creator, Error::<T>::BadOrigin);
            Ok(dao.creator)
        }

        /// 获取创建者
        pub fn ensrue_root_guild(
            who: T::AccountId,
            dao_id: DaoAssetId,
            guild_id: u64,
        ) -> result::Result<T::AccountId, DispatchError> {
            let guild = <Guilds<T>>::get(dao_id);
            ensure!(!guild.is_empty(), Error::<T>::BadOrigin);

            let gindex: u64 = guild_id;
            let members = <GuildMembers<T>>::get(dao_id, gindex);
            members
                .binary_search(&who)
                .err()
                .ok_or(Error::<T>::BadOrigin)?;

            Ok(who)
        }

        /// 添加成员
        pub fn try_add_guild_member(
            dao_id: DaoAssetId,
            guild_id: u64,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let guild = <Guilds<T>>::get(dao_id);
            ensure!(!guild.is_empty(), Error::<T>::BadOrigin);

            let gindex: u64 = guild_id;
            let mut members = <GuildMembers<T>>::get(dao_id, gindex);
            let index = members
                .binary_search(&who)
                .err()
                .ok_or(Error::<T>::InVailCall)?;

            members
                .try_insert(index, who)
                .map_err(|_| Error::<T>::TooManyMembers)?;

            <GuildMembers<T>>::insert(dao_id, gindex, &members);

            Ok(index)
        }

        /// 删除成员
        pub fn try_remove_guild_member(
            dao_id: DaoAssetId,
            guild_id: u64,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let guild = <Guilds<T>>::get(dao_id);
            ensure!(!guild.is_empty(), Error::<T>::BadOrigin);

            let gindex: u64 = guild_id;
            let mut members = <GuildMembers<T>>::get(dao_id, gindex);
            let index = members
                .binary_search(&who)
                .ok()
                .ok_or(Error::<T>::InVailCall)?;

            members.remove(index);
            <GuildMembers<T>>::insert(dao_id, gindex, &members);

            Ok(index)
        }

        /// 添加成员
        pub fn try_add_member(
            dao_id: DaoAssetId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            // 初始化成员
            let mut members = <Members<T>>::get(dao_id);
            let index = members
                .binary_search(&who)
                .err()
                .ok_or(Error::<T>::InVailCall)?;
            members
                .try_insert(index, who)
                .map_err(|_| Error::<T>::GuildCreateError)?;

            <Members<T>>::insert(dao_id, &members);
            Ok(index)
        }

        /// 删除成员
        pub fn try_remove_member(
            dao_id: DaoAssetId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let mut members = <Members<T>>::get(dao_id);
            let index = members
                .binary_search(&who)
                .ok()
                .ok_or(Error::<T>::InVailCall)?;

            members.remove(index);
            <Members<T>>::insert(dao_id, &members);

            Ok(index)
        }

        /// 添加项目成员
        pub fn try_add_project_member(
            dao_id: DaoAssetId,
            project_id: ProjectId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let gindex: u64 = project_id;
            let mut members = <ProjectMembers<T>>::try_get(dao_id, gindex).unwrap_or_default();
            let index = members
                .binary_search(&who)
                .err()
                .ok_or(Error::<T>::InVailCall)?;

            members
                .try_insert(index, who)
                .map_err(|_| Error::<T>::TooManyMembers)?;

            <ProjectMembers<T>>::insert(dao_id, gindex, &members);

            Ok(index)
        }

        /// 删除成员
        pub fn try_remove_project_member(
            dao_id: DaoAssetId,
            project_id: ProjectId,
            who: T::AccountId,
        ) -> result::Result<usize, DispatchError> {
            let gindex: u64 = project_id;
            let mut members = <ProjectMembers<T>>::try_get(dao_id, gindex).unwrap_or_default();
            let index = members
                .binary_search(&who)
                .ok()
                .ok_or(Error::<T>::InVailCall)?;

            members.remove(index);
            <ProjectMembers<T>>::insert(dao_id, gindex, &members);

            Ok(index)
        }

        /// 为成员添加荣誉积分
        pub fn try_add_member_point(
            dao_id: DaoAssetId,
            who: T::AccountId,
            point: u32,
        ) -> result::Result<u32, DispatchError> {
            let mut p = <MemberPoint<T>>::get(dao_id, who.clone());
            p += point;

            <MemberPoint<T>>::insert(dao_id, who, p);

            Ok(p)
        }

        /// 获取任务列表
        pub fn get_task(
            dao_id: DaoAssetId,
            roadmap_id: u32,
            task_id: TaskId,
        ) -> result::Result<
            (
                BoundedVec<QuarterTask<T::AccountId>, ConstU32<100>>,
                QuarterTask<T::AccountId>,
                usize,
            ),
            DispatchError,
        > {
            // 获取人物列表
            let tasks = <RoadMaps<T>>::try_get(dao_id, roadmap_id).unwrap_or_default();
            let index = tasks
                .binary_search_by(|t| t.id.cmp(&task_id))
                .ok()
                .ok_or(Error::<T>::InVailCall)?;

            // 获取原始任务
            let task_brow = tasks.get(index).unwrap();
            let task = task_brow.clone();

            Ok((tasks, task, index))
        }
    }
}
