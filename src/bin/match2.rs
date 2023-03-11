fn main() {
    let fd_maturity_period = 12;

    if fd_maturity_period == 3 {
        println!("3 month fixed deposit");
    } else if fd_maturity_period == 12 {
        println!("12 month fixed deposit");
    } else {
        println!("Unknown maturity period");
    }

    match fd_maturity_period {
        // identical control flow using match
        3 => println!("3 month fixed deposit"),
        12 => println!("12 month fixed deposit"),
        _ => println!("Unknown maturity period"), // everything else (exhaustive)
    }
}
