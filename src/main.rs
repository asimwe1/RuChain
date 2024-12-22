use std::alloc::System;
mod balances;
mod system;
mod transfer;

#[derive(Debug)]
pub struct Runtime{
    system: system::Pallet,
    balances: balances::Pallet,

}
impl Runtime{
    pub fn new() -> Self{
        Self{
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }
}


fn main() {
    let mut runtime = Runtime::new();
    let alice = String::from("Alice");
    let bob = String::from("Bob");
    let charlie = String::from("Charlie");

    runtime.balances.set_balance(alice.clone(), 100);
    runtime.system.inc_block_number();

    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&alice);
    let _ = runtime.balances
        .transfer(alice.clone(), bob.clone(), 30)
        .map_err(|e|{println!("ERR: {:#?}", e)});

    let _ = runtime.balances
        .transfer(alice.clone(), charlie.clone(), 50)
        .map_err(|e|{println!("ERR: {:#?}", e)});

    let _ = runtime.balances
        .transfer(bob.clone(), charlie.clone(), 5)
        .map_err(|e|{println!("Err: {:#?}", e)});

    println!("{:#?}", runtime);

    // match _ {
    //     Ok(_)=>{
    //         println!("transfer OK/successful");
    // }
    //     Err(e)=>{
    //         println!("transfer FAILED/NOT SUCCESSFULLY: {:?} ", e);
    //     }
    //
    // }

}

#[test]
fn init_balances() {

    let mut balances = balances::Pallet::new();
    let mut system = system::Pallet::new();

    assert_eq!(balances.balance(&"alice".to_string()), 0);

    balances.set_balance("alice".to_string(), 100);

    assert_eq!(balances.balance(&"alice".to_string()), 100);
    assert_eq!(balances.balance(&"bob".to_string()), 0);
}

#[test]
fn fail_test() {
    assert_eq!(2 , 2);
}
