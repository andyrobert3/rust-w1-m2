fn main() {
    let vec1: Vec<i32> = Vec::new(); // Declare vector to hold i32
    println!("vec1 is {:?}", vec1);

    let vec2 = vec![1, 2, 3]; // Declare vector, Rust will infer a i32 Vector
    println!("vec2 is {:?}", vec2); // prints "v is [1, 2, 3]"

    let mut vec3 = Vec::with_capacity(5); // Declare vector with capacity of 5 at the start vec3 = []
    vec3.push(50); // vec3 = [50]
    println!("vec3 is {:?}", vec3); // prints "vec is [50]"
    println!("vec3 length is :{}", vec3.len()); // prints "vec length is: 1"
    println!("vec3 capacity is :{}", vec3.capacity()); // prints "vec capacity is: 5"
}
