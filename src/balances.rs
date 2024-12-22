use std::collections::BTreeMap;

#[derive(Debug
)]
pub struct Pallet {
    balances: BTreeMap<String, u128>,
}


impl Pallet {

    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    // setting balance for user
    pub(crate) fn set_balance(&mut self, who: String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    //getting balance for who
    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    //balance transfer
    pub fn transfer(&mut self, caller: String, to: String, amount: u128) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);
        let new_sender_balance = caller_balance
            .checked_sub(amount)
            .ok_or("insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("overflow when adding balance")?;

        self.balances.insert(caller.clone(), new_sender_balance);
        self.balances.insert(to.clone(), new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::balances::Pallet;

    #[test]
    fn set_balance() {
        let mut balances = Pallet::new();

        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance("alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);

    }

    #[test]
    fn transfer_balance() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balances = super::Pallet::new();
        assert_eq!(balances.balance(&alice), 0);
        balances.set_balance("alice".to_string(), 100);
        balances.transfer(alice.clone(), bob.clone() , 90).unwrap();

        assert_eq!(balances.balance(&alice), 10);
        assert_eq!(balances.balance(&bob), 90);
    }

    #[test]
    fn transfer_balance_insufficient() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        let mut balances = super::Pallet::new();
        balances.set_balance(alice.clone(), 100);
        let result = balances.transfer(alice.clone(), bob.clone(), 110);

        assert_eq!(result, Err("insufficient balance"));
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), 0);

    }

    #[test]
    fn transfer_balance_overflow() {
        let alice = "alice".to_string();
        let bob = "bob".to_string();
        let mut balances = super::Pallet::new();

        balances.set_balance(alice.clone(), 100);
        balances.set_balance(bob.clone(), u128::MAX);
        let result = balances.transfer(alice.clone(), bob.clone(), 1);
        assert_eq!(result, Err("overflow when adding balance"));
        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), u128::MAX);
    }
}

#[test]
fn set_transfer_balance() {
    let mut balances = Pallet::new();
    assert_eq!(balances.balance(&"alice".to_string()), 0);
    println!("balance: {}", balances.balance(&"bob".to_string()));
}