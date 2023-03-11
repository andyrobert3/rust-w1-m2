fn main() {
    let my_vector = vec![1, 2, 3];

    for element in &my_vector {
        // for loop borrows reference to my_vector
        println!("I like {}.", element);
    }
    println!("{:?}", my_vector); // this works
}
