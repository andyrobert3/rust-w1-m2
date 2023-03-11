fn main() {
    let mut x = "vincent".to_owned();
    let y = x; // “vincent” ownership is moved to y
    x = "justin".to_owned(); // will this line error?
    println!("x is {}", x); // what will this print?
    println!("y is {}", y); // what will this print?
}
