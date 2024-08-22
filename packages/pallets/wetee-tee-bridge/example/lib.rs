#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod ext;
use crate::ext::*;

#[ink::scale_derive(Encode, Decode, TypeInfo)]
struct CallBackData {
    pub v: u128,
    pub func_id: u8,
}

#[ink::contract(env = crate::ext::TbExtEnvironment)]
mod test_contract {
    use super::{CallBackData, TbExtErr};

    #[ink(storage)]
    pub struct TStore {
        value: u128,
    }

    #[ink(event)]
    pub struct InkUpdated {
        #[ink(topic)]
        new: u128,
    }

    impl TStore {
        #[ink(constructor)]
        pub fn new(init_value: u128) -> Self {
            Self { value: init_value }
        }

        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn update(&mut self) -> Result<(), TbExtErr> {
            self.env()
                .extension()
                .call_tee(
                    crate::ext::WorkId {
                        wtype: crate::ext::WorkType::App,
                        id: 0,
                    },
                    1,
                    [0, 0, 0, 42],
                    [0; 500],
                )
                .unwrap();

            Ok(())
        }

        #[ink(message, selector = 42)]
        pub fn callback(&mut self, v: Vec<u8>) -> Result<(), TbExtErr> {
            let mut data = v.as_slice();
            // let mut call_args = CallBackData { func_id: 0, v: 0 };

            // 使用 decode_into 方法将字节数据解码到结构体中
            let call_args: CallBackData = ink::scale::Decode::decode(&mut data)?;

            self.value = call_args.v;
            self.env().emit_event(InkUpdated { new: call_args.v });
            Ok(())
        }

        #[ink(message)]
        pub fn get(&self) -> u128 {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let ink_extension = TStore::new_default();
            assert_eq!(ink_extension.get(), 0);
        }

        #[ink::test]
        fn callback_work() {
            let mut ink_extension = TStore::new_default();
            assert_eq!(ink_extension.get(), 0);

            let ret = CallBackData { func_id: 0, v: 1 };
            let output = &mut Vec::new();
            ink::scale::Encode::encode_to(&ret, output);

            // when
            ink_extension
                .callback(output.to_vec())
                .expect("update must work");

            // then
            assert_eq!(ink_extension.get(), 1);
        }
    }
}
