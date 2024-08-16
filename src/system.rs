use std::collections::BTreeMap;
use num::traits::{Zero, One};
use core::ops::AddAssign;

pub trait Config {
    type AccountId : Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy;
    type Nonce: Zero + One + Copy;
}


#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T>
{
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&mut self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += T::BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero()) + T::Nonce::one();
        self.nonce.insert(who.clone(), nonce);
    }
}

#[cfg(test)]
mod test {
    struct TestConfig;
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }
    #[test]
    fn init_system() {
        let mut system = super::Pallet::<TestConfig>::new();

        assert_eq!(system.block_number(), 0);
        assert_eq!(system.nonce.get(&"alice".to_string()), None);

        system.inc_block_number();
        assert_eq!(system.block_number(), 1);

        system.inc_nonce(&"alice".to_string());
        assert_eq!(system.nonce.get(&"alice".to_string()).unwrap(), &1);
    }
}
