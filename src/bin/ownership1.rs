fn main() {
    // Scalar data type ownership example
    let x = 10; // x is the owner of value "10" in memory
    let y = x; // This copies the value in memory
    println!("{}", x); // this works, x is owner of its own "10"
    println!("{}", y); // this works, y is owner of its own "10"

    // Compound data type ownership example
    let s1 = "abc".to_owned(); // s1 is owner of value "abc" in memory
    let s2 = s1.clone(); // Value of "abc" is cloned in memory
                         // Also, s1 is not automatically cleaned up anymore
    println!("{:?}", s1); // this works now
}
