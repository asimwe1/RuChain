pub struct Transfer {
    from: String,
    to: String,
    Amount: u128,
}

impl Transfer {
    let user1 = String::from("Eric");
    let user2 = String::from("Aaron");
    pub fn new(from: &str, to: &str, Amount: u128) -> Self {
        Self{
            from,
            to,
            Amount,

        }
    }
}