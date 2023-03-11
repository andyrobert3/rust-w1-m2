fn main() {
    let a = 10;
    let b = a; // this copies the value on stack, and does not move ownership
    println!("a is {}", a); // this works, prints 10
    println!("b is {}", b); // this works too, prints 10
}
