fn main() {
    let s1 = "abc".to_owned(); // s1 is owner of value "abc"
    let s2 = s1; // Ownership of value in s1 is moved to s2
    println!("{:?}", s1); // Error!
}
