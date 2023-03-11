fn main() {
    let mut v = Vec::new(); // []

    v.push(1); // [1]
    v.push(2); // [1, 2]
    v.push(3); // [1, 2, 3]
    v.push(4); // [1, 2, 3, 4]

    v.pop(); // removes the top element "4" → [1, 2, 3]
    println!("{:?}", v); // prints "[1, 2, 3]"
}
