fn main() {
    let mut v = vec![1, 2, 3, 4];

    println!("Vector: {:?}", v);

    let index_to_remove = 2;
    if index_to_remove < v.len() {
        v.remove(index_to_remove);
        println!("After remove: {:?}", v);
    }

    let index_to_insert = 1;
    if index_to_remove < v.len() {
        v.insert(index_to_insert, 9);
        println!("After insert: {:?}", v);
    }
}
