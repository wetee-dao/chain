use ink::env::Environment;
// use ink::prelude::vec::Vec;

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
    ArgErr,
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

// pub trait InkVec {
//     fn vec_decode(input: Vec<u8>) -> Result<Self, TbExtErr>
//     where
//         Self: Sized;
//     fn vec_encode(&self) -> Vec<u8>;
// }

// impl InkVec for u128 {
//     fn vec_decode(input: Vec<u8>) -> Result<Self, TbExtErr>
//     where
//         Self: Sized,
//     {
//         if input.len() != 16 {
//             return Err(crate::TbExtErr::ArgErr);
//         }
//         let mut r = [0u8; 16];
//         r.copy_from_slice(&input);
//         Ok(u128::from_le_bytes(r))
//     }

//     fn vec_encode(&self) -> Vec<u8> {
//         self.to_le_bytes().to_vec()
//     }
// }

// impl InkVec for i128 {
//     fn vec_decode(input: Vec<u8>) -> Result<Self, TbExtErr>
//     where
//         Self: Sized,
//     {
//         if input.len() != 16 {
//             return Err(crate::TbExtErr::ArgErr);
//         }
//         let mut r = [0u8; 16];
//         r.copy_from_slice(&input);
//         Ok(i128::from_le_bytes(r))
//     }

//     fn vec_encode(&self) -> Vec<u8> {
//         self.to_le_bytes().to_vec()
//     }
// }

// impl InkVec for u64 {
//     fn vec_decode(input: Vec<u8>) -> Result<Self, TbExtErr>
//     where
//         Self: Sized,
//     {
//         if input.len() != 8 {
//             return Err(crate::TbExtErr::ArgErr);
//         }
//         let mut r = [0u8; 8];
//         r.copy_from_slice(&input);
//         Ok(u64::from_le_bytes(r))
//     }

//     fn vec_encode(&self) -> Vec<u8> {
//         self.to_le_bytes().to_vec()
//     }
// }

// impl InkVec for i64 {
//     fn vec_decode(input: Vec<u8>) -> Result<Self, TbExtErr>
//     where
//         Self: Sized,
//     {
//         if input.len() != 8 {
//             return Err(crate::TbExtErr::ArgErr);
//         }
//         let mut r = [0u8; 8];
//         r.copy_from_slice(&input);
//         Ok(i64::from_le_bytes(r))
//     }

//     fn vec_encode(&self) -> Vec<u8> {
//         self.to_le_bytes().to_vec()
//     }
// }

// impl InkVec for u32 {
//     fn vec_decode(input: Vec<u8>) -> Result<Self, TbExtErr>
//     where
//         Self: Sized,
//     {
//         if input.len() != 4 {
//             return Err(crate::TbExtErr::ArgErr);
//         }
//         let mut r = [0u8; 4];
//         r.copy_from_slice(&input);
//         Ok(u32::from_le_bytes(r))
//     }

//     fn vec_encode(&self) -> Vec<u8> {
//         self.to_le_bytes().to_vec()
//     }
// }

// impl InkVec for i32 {
//     fn vec_decode(input: Vec<u8>) -> Result<Self, TbExtErr>
//     where
//         Self: Sized,
//     {
//         if input.len() != 4 {
//             return Err(crate::TbExtErr::ArgErr);
//         }
//         let mut r = [0u8; 4];
//         r.copy_from_slice(&input);
//         Ok(i32::from_le_bytes(r))
//     }

//     fn vec_encode(&self) -> Vec<u8> {
//         self.to_le_bytes().to_vec()
//     }
// }

// impl InkVec for u16 {
//     fn vec_decode(input: Vec<u8>) -> Result<Self, TbExtErr>
//     where
//         Self: Sized,
//     {
//         if input.len() != 2 {
//             return Err(crate::TbExtErr::ArgErr);
//         }
//         let mut r = [0u8; 2];
//         r.copy_from_slice(&input);
//         Ok(u16::from_le_bytes(r))
//     }

//     fn vec_encode(&self) -> Vec<u8> {
//         self.to_le_bytes().to_vec()
//     }
// }

// impl InkVec for i16 {
//     fn vec_decode(input: Vec<u8>) -> Result<Self, TbExtErr>
//     where
//         Self: Sized,
//     {
//         if input.len() != 2 {
//             return Err(crate::TbExtErr::ArgErr);
//         }
//         let mut r = [0u8; 2];
//         r.copy_from_slice(&input);
//         Ok(i16::from_le_bytes(r))
//     }

//     fn vec_encode(&self) -> Vec<u8> {
//         self.to_le_bytes().to_vec()
//     }
// }

// impl InkVec for u8 {
//     fn vec_decode(input: Vec<u8>) -> Result<Self, TbExtErr>
//     where
//         Self: Sized,
//     {
//         if input.len() != 1 {
//             return Err(crate::TbExtErr::ArgErr);
//         }
//         Ok(input[0])
//     }

//     fn vec_encode(&self) -> Vec<u8> {
//         self.to_le_bytes().to_vec()
//     }
// }

// impl InkVec for i8 {
//     fn vec_decode(input: Vec<u8>) -> Result<Self, TbExtErr>
//     where
//         Self: Sized,
//     {
//         if input.len() != 1 {
//             return Err(crate::TbExtErr::ArgErr);
//         }
//         let mut r = [0u8; 1];
//         r.copy_from_slice(&input);
//         Ok(i8::from_le_bytes(r))
//     }

//     fn vec_encode(&self) -> Vec<u8> {
//         self.to_le_bytes().to_vec()
//     }
// }
