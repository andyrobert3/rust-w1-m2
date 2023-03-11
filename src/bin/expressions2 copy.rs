fn main() {
    let mut i = 1;
    // loop written here as an expression on the right side of the = sign
    let something = loop {
        i *= 2;
        println!("i: {}", i);
        if i > 100 {
            break i;
        }
    };
}
