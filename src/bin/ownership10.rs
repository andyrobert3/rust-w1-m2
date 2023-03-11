fn main() {
    let a = vec![1, 2, 3];
    let mut b = a; // ownership of vector transferred from a to b

    a.push(4); // will this work? why?
    println!("a: {:?}", a); // will this work? why?

    b.push(4); // will this work? why?
    println!("b: {:?}", b); // will this work? why?
}
