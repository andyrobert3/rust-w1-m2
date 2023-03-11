fn main() {
    let s1 = "abc".to_owned();
    let s2 = s1.clone();
    println!("{:?}", s1); // Works!
}
