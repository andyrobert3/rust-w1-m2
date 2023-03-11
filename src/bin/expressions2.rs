fn main() {
    let x = 7;

    // match written here as an expression on the right side of the = sign
    let message = match x {
        1 => "hello",
        _ => "goodbye",
    };

    println!("message: {}", message);
}
