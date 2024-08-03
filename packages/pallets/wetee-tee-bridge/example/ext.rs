use ink::env::Environment;

#[ink::chain_extension(extension = 2)]
pub trait TbExt {
    type ErrorCode = TbExtErr;

    #[ink(function = 1001)]
    fn call_tee(
        tee: WorkId,
        method: u16,
        callback_method: [u8; 4],
        args: [u8; 500],
    ) -> Result<u128, TbExtErr>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum TbExtErr {
    FailGetInkomSource,
}

impl From<ink::scale::Error> for TbExtErr {
    fn from(_: ink::scale::Error) -> Self {
        panic!("encountered unexpected invalid SCALE encoding")
    }
}

impl ink::env::chain_extension::FromStatusCode for TbExtErr {
    fn from_status_code(status_code: u32) -> Result<(), Self> {
        match status_code {
            0 => Ok(()),
            1 => Err(Self::FailGetInkomSource),
            _ => panic!("encountered unknown status code"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[ink::scale_derive(TypeInfo)]
pub enum TbExtEnvironment {}

impl Environment for TbExtEnvironment {
    const MAX_EVENT_TOPICS: usize = <ink::env::DefaultEnvironment as Environment>::MAX_EVENT_TOPICS;

    type AccountId = <ink::env::DefaultEnvironment as Environment>::AccountId;
    type Balance = <ink::env::DefaultEnvironment as Environment>::Balance;
    type Hash = <ink::env::DefaultEnvironment as Environment>::Hash;
    type BlockNumber = <ink::env::DefaultEnvironment as Environment>::BlockNumber;
    type Timestamp = <ink::env::DefaultEnvironment as Environment>::Timestamp;

    type ChainExtension = TbExt;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum WorkType {
    /// APP
    App = 0,
    /// TASK
    Task,
    /// GPU
    Gpu,
}

/// WorkId
/// 工作ID
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub struct WorkId {
    pub wtype: WorkType,
    pub id: u64,
}
