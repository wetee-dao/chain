#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod ext;
use crate::ext::*;
use ink::prelude::vec::Vec;

#[ink::contract(env = TbExtEnvironment)]
mod test_contract {
    use super::*;

    #[ink(storage)]
    pub struct TStore {
        value: u128,
        booolv: bool,
        stringv: Vec<u8>,
    }

    #[ink(event)]
    pub struct InkUpdated {
        #[ink(topic)]
        new: u128,
    }

    impl TStore {
        #[ink(constructor)]
        pub fn new(init_value: u128) -> Self {
            Self {
                value: init_value,
                booolv: true,
                stringv: Vec::new(),
            }
        }

        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn call_tee(&mut self, tee_id: u64) -> Result<(), TbExtErr> {
            let result = self.env().extension().call_tee(
                WorkId {
                    wtype: WorkType::App,
                    id: tee_id,
                },
                0,
                [0, 0, 0, 42],
                [0; 500],
            );

            match result {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }

        #[ink(message, selector = 42)]
        pub fn callback(&mut self, v: u128, b: bool, s: Vec<u8>) -> Result<(), TbExtErr> {
            self.value = v;
            self.booolv = b;
            self.stringv = s;
            self.env().emit_event(InkUpdated { new: v });
            Ok(())
        }

        #[ink(message)]
        pub fn get(&self) -> u128 {
            self.value
        }

        #[ink(message)]
        pub fn getb(&self) -> bool {
            self.booolv
        }

        #[ink(message)]
        pub fn gets(&self) -> Vec<u8> {
            self.stringv.clone()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let ink_extension = TStore::new_default();
            assert_eq!(ink_extension.get(), 0);
        }

        #[ink::test]
        fn callback_work() {
            let mut ink_extension = TStore::new_default();
            assert_eq!(ink_extension.get(), 0);

            // when
            ink_extension
                .callback(100, true, "test".as_bytes().to_vec())
                .expect("update must work");

            // then
            assert_eq!(ink_extension.get(), 100);
            assert_eq!(ink_extension.getb(), true);
            assert_eq!(ink_extension.gets(), "test".as_bytes().to_vec());
        }
    }
}
