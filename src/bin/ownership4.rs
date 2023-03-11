fn main() {
    {
        let x = 10; // x is valid from this point forward
        println!("x is {}", x);
    }
    // scope is now over, and x is no longer valid
    println!("x is {}", x); // Error as x no longer exists
}
