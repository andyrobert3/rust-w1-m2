// Create a User struct
// with fields balance(f32), currency(String)
struct User {
    balance: f32,
    currency: String,
}

fn main() {
    // In main, create a User instance with balance of 100.00, currency of "SGD"
    let user = User {
        balance: 100.0,
        currency: "SGD".to_owned(),
    };

    // Using expression and match, assign balance_is_100 to true if balance equals to 100.00. Or else, return false.
    let balance_is_100: bool = match user.balance {
        100.00 => true,
        _ => false,
    };

    // Print the value of balance_is_100
    println!("balance_is_100: {}", balance_is_100);
}
