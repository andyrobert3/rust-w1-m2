fn main() {
    let sufficient_funds = true;

    if sufficient_funds == true {
        println!("Purchase Successful");
    } else if sufficient_funds == false {
        println!("Purchase Failed");
    }

    match sufficient_funds {
        // identical control flow using match
        true => println!("Purchase Successful"),
        false => println!("Purchase Failed"),
    }
}
