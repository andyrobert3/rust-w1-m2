fn main() {
    let mut a = "abc".to_owned();
    add_d(&mut a); // (1) ownership of value moved to the function
    println!("a is {} ", a); // (3) works!
}
fn add_d(x: &mut String) {
    println!("do_something received ptr: {:p}, value: {}", x, *x);
    *x = "abcd".to_owned();
} // (2) scope end, ownership is returned to main()
