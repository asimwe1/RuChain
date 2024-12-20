mod balances;
mod system;

fn main() {
    println!("Hello, world!");
}

#[test]
fn init_balances() {

    let mut balances = balances::Pallet::new();
    let system = system::Pallet::new();

    assert_eq!(balances.balance(&"alice".to_string()), 0);

    balances.set_balance("alice".to_string(), 100);

    assert_eq!(balances.balance(&"alice".to_string()), 100);
    assert_eq!(balances.balance(&"bob".to_string()), 0);
}

#[test]
fn fail_test() {
    assert_eq!(2 , 2);
}




// let mut pallet = balances::Pallet::new();
// map.insert("alice", 100);
// assert_eq!(map.get(&"alice").unwrap_or(&0), &100))
// assert_eq!(map.get(&"bob"), None);
//
// let maybe_value = map.get(&"alice");
// match maybe_value {
// Some(value) => {
// //do something
// },
// None => {
// println!("{}", value);
// }
// }
