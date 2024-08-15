use std::collections::BTreeMap;
use num::traits::{Zero, One};
use core::ops::AddAssign;

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>,
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce>
where
    AccountId: Ord + Clone,
    BlockNumber: Zero + One + AddAssign + Copy,
    Nonce: Zero + One + Copy  

{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&mut self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += BlockNumber::one();
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero()) + Nonce::one();
        self.nonce.insert(who.clone(), nonce);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let mut system = super::Pallet::<String, u32, u32>::new();

        assert_eq!(system.block_number(), 0);
        assert_eq!(system.nonce.get(&"alice".to_string()), None);

        system.inc_block_number();
        assert_eq!(system.block_number(), 1);

        system.inc_nonce(&"alice".to_string());
        assert_eq!(system.nonce.get(&"alice".to_string()).unwrap(), &1);
    }
}
