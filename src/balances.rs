use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: &String,
        to: &String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let amount_caller = self.balance(&caller);
        let amount_to = self.balance(&to);
        let new_caller_balance = amount_caller
            .checked_sub(amount)
            .ok_or("Not enough funds.")?;
        let new_to_balance = amount_to.checked_add(amount).ok_or("Overflow occured")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_balances() {
        let mut balances = Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);

        balances.set_balance(&"alice".to_string(), 100);

        assert_eq!(balances.balance(&"alice".to_string()), 100);

        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::new();

        assert_eq!(
            balances.transfer(&"alice".to_string(), &"bob".to_string(), 100),
            Err("Not enough funds.")
        );

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(
            balances.transfer(&"alice".to_string(), &"bob".to_string(), 100),
            Ok(())
        );

        assert_eq!(balances.balance(&"alice".to_string()), 0);

        assert_eq!(balances.balance(&"bob".to_string()), 100);
 
    }
}
