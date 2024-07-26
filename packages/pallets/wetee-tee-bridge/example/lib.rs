#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod ext;
use crate::ext::*;

#[ink::contract(env = crate::ext::TbExtEnvironment)]
mod test_contract {
    use super::BtExtErr;

    #[ink(storage)]
    pub struct TStore {
        /// Stores a single `bool` value on the storage.
        value: [u8; 32],
    }

    #[ink(event)]
    pub struct InkUpdated {
        #[ink(topic)]
        new: [u8; 32],
    }

    impl TStore {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: [u8; 32]) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors may delegate to other constructors.
        #[ink(constructor)]
        pub fn new_default() -> Self {
            Self::new(Default::default())
        }

        /// Seed a inkom value by passing some known argument `subject` to the runtime's
        /// inkom source. Then, update the current `value` stored in this contract with
        /// the new inkom value.
        #[ink(message)]
        pub fn update(&mut self, subject: [u8; 32]) -> Result<(), BtExtErr> {
            // Get the on-chain inkom seed
            let new_inkom = self.env().extension().test_func(subject)?;
            self.value = new_inkom;
            // Emit the `InkUpdated` event when the inkom seed
            // is successfully fetched.
            self.env().emit_event(InkUpdated { new: new_inkom });
            Ok(())
        }

        /// Simply returns the current value.
        #[ink(message)]
        pub fn get(&self) -> [u8; 32] {
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
            struct MockedBtExtension;
            impl ink::env::test::ChainExtension for MockedBtExtension {
                /// The static function id of the chain extension.
                fn ext_id(&self) -> u16 {
                    666
                }

                /// The chain extension is called with the given input.
                ///
                /// Returns an error code and may fill the `output` buffer with a
                /// SCALE encoded result. The error code is taken from the
                /// `ink::env::chain_extension::FromStatusCode` implementation for
                /// `BtExtErr`.
                fn call(
                    &mut self,
                    _func_id: u16,
                    _input: &[u8],
                    output: &mut Vec<u8>,
                ) -> u32 {
                    let ret: [u8; 32] = [1; 32];
                    ink::scale::Encode::encode_to(&ret, output);
                    0
                }
            }
            ink::env::test::register_chain_extension(MockedBtExtension);
            let mut ink_extension = BtExtension::new_default();
            assert_eq!(ink_extension.get(), [0; 32]);

            // when
            ink_extension.update([0_u8; 32]).expect("update must work");

            // then
            assert_eq!(ink_extension.get(), [1; 32]);
        }
    }
}