fn main() {
    let v = vec![1, 2, 3, 4, 5];

    // Use indexing - program will panic / crash if accessing out of index
    println!("3rd Element is {}", v[2]); // prints 3
                                         // Use get method, to gracefully handle an out of index situation
    match v.get(3) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    // Some(third) is an Option Type that will be explained in chapter "Option"
}
