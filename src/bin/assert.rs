fn main() {
    let a = 3;
    let b = 1 + 1;

    // ensure both values are equal, otherwise panic
    assert_eq!(a, b);

    // ensure both values are equal, otherwise panic with a custom error message
    assert_eq!(a, b, "we are testing addition with {} and {}", a, b);

    // ensure both values are not equal, otherwise panic
    assert_ne!(a, b);
}
