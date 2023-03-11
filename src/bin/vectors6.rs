fn main() {
    let v = vec![10, 20, 30];
    for i in v {
        // v ownership is moved to i
        // for i in &v {	// you should really be using & to "borrow" v instead
        println!("{}", i);
    }

    for i in v {
        // Error - Will not compile as v no longer exists
        println!("{}", i);
    }
}
