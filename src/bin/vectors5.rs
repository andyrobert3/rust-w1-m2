fn main() {
    // Iterate over a immutable reference to each Vector element
    let v = vec![10, 20, 30];
    for i in &v {
        // & to borrow the value, otherwise ownership is passed to for loop
        println!("{}", i);
    }

    // Iterate over a mutable reference to each Vector element
    let mut v = vec![10, 20, 30];
    for i in &mut v {
        *i += 1; // use the * dereference operator to modify the value
        println!("{}", i);
    }
}
