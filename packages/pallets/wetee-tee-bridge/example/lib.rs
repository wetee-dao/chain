#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod ext;
use crate::ext::*;

#[ink::contract(env = crate::ext::TbExtEnvironment)]
mod test_contract {
    use super::TbExtErr;

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
            let res = self
                .env()
                .extension()
                .call_tee(
                    crate::ext::WorkId {
                        wtype: crate::ext::WorkType::App,
                        id: 0,
                    },
                    1,
                    [0; 500],
                )
                .unwrap();
            self.value = res;
            self.env().emit_event(InkUpdated { new: res });
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
            assert_eq!(ink_extension.get(), [0; 32]);
        }

        #[ink::test]
        fn chain_extension_works() {
            // given
            struct MockedTbExtension;
            impl ink::env::test::ChainExtension for MockedTbExtension {
                /// The static function id of the chain extension.
                fn ext_id(&self) -> u16 {
                    666
                }

                /// The chain extension is called with the given input.
                ///
                /// Returns an error code and may fill the `output` buffer with a
                /// SCALE encoded result. The error code is taken from the
                /// `ink::env::chain_extension::FromStatusCode` implementation for
                /// `TbExtErr`.
                fn call(&mut self, _func_id: u16, _input: &[u8], output: &mut Vec<u8>) -> u32 {
                    let ret: [u8; 32] = [1; 32];
                    ink::scale::Encode::encode_to(&ret, output);
                    0
                }
            }
            ink::env::test::register_chain_extension(MockedTbExtension);
            let mut ink_extension = TbExtension::new_default();
            assert_eq!(ink_extension.get(), [0; 32]);

            // when
            ink_extension.update([0_u8; 32]).expect("update must work");

            // then
            assert_eq!(ink_extension.get(), [1; 32]);
        }
    }
}
