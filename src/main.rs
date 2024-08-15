mod balances;
mod system;

mod types {
    pub type Balance = u128;
    pub type AccountId = String;
    pub type BlockNumber = u64;
    pub type Nonce = u32;
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<types::AccountId, types::BlockNumber, types::Nonce>,
    balances: balances::Pallet<types::AccountId, types::Balance>,
}

impl Runtime {
    fn new() -> Self { 
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}

fn main() {
    let mut runtime = Runtime::new();
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);

    runtime.system.inc_block_number();
    assert!(runtime.system.block_number() == 1);

    runtime.system.inc_nonce(&alice);

    let _res = runtime.balances.transfer(alice.clone(), bob, 30).map_err(|e| eprintln!("{}", e));

    runtime.system.inc_nonce(&alice);

    let _res = runtime.balances.transfer(alice, charlie, 20).map_err(|e| eprintln!("{}", e));

    println!("{:#?}", runtime);
}
