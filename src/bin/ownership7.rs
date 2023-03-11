fn main() {
    let a = "abc".to_owned();
    do_something(&a); // (1) ownership of value moved to the function
    println!("a is {} ", a); // (3) works!
}

fn do_something(x: &String) {
    println!("do_something received {} ", x);
} // (2) scope end, ownership is returned to main()
