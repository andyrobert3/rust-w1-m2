struct User {
    name: String,
    balance: (f32, String),
}

impl User {
    fn print_user_detail(&self) {
        println!("Name: {}, Balance: {:?}", self.name, self.balance)
    }
}

fn main() {
    let user = User {
        name: "John".to_owned(),
        balance: (100.0, "SGD".to_owned()),
    };

    accrue_interest(&user, 10.0);
    accrue_interest(&user, 10.0);
}

fn accrue_interest(user: &User, interest: f32) {
    user.balance.0 = user.balance.0 + (user.balance.0 * interest / 100.0);
    user.print_user_detail();
}
