use core::fmt::Debug;
use std::collections::BTreeMap;
use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

pub enum Call<T: Config> {
    CreateClaim { claim: T::Content},
    RevokeClaim { claim: T::Content},
}


#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(
        &mut self,
        caller: Self::Caller,
        call: Self::Call
    ) -> crate::support::DispatchResult{
        match call {
            Call::CreateClaim { claim } => {
                self.create_claim(caller, claim)?
            }
            Call::RevokeClaim { claim } => {
                self.revoke_claim(caller, claim)?
            }
        }
        Ok(())
    }


}


impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.claims.contains_key(&claim) {
            return Err(&"this content is already claimed");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let owner = self.claims.get(&claim).ok_or("no claim found")?;
        if owner != &caller {
            return Err(&"this claim is owned by someone else");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::proof_of_existence::Pallet;

    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence(){
        let alice = "alice";
        let bob = "bob";
        let mut poe = Pallet::<TestConfig>::new();
        assert!(poe.get_claim(&"Hello").is_none());

        let _ = poe.create_claim(alice, "Hello");

        assert!(poe.create_claim(bob, "Hello").is_err())

    }

    #[test]
    fn revoke(){
        let alice = "alice";
        let bob = "bob";
        let mut poe = Pallet::<TestConfig>::new();
        let _ = poe.create_claim(alice, "Hello");
        
        assert!(poe.revoke_claim(bob, "Hello").is_err());
        assert!(poe.revoke_claim(alice, "Hello").is_ok());
        assert!(poe.get_claim(&"Hello").is_none());

        assert!(poe.revoke_claim(bob, "No Claim Exist").is_err());
    }

    #[test]
    fn cant_claim_existing_claim(){
        let alice = "alice";
        let mut poe = Pallet::<TestConfig>::new();
        assert!(poe.get_claim(&"Hello").is_none());

        let _ = poe.create_claim(alice, "Hello");

        assert_eq!(poe.get_claim(&"Hello"), Some(&alice));
    }

}

