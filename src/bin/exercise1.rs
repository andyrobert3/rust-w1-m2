fn main() {
    let user_vec: Vec<String> = vec![
        "john".to_owned(),
        "mary".to_owned(),
        "simon".to_owned(),
        "john".to_owned(),
        "kelly".to_owned(),
        "harry".to_owned(),
    ];

    let mut counter = 0;
    // TODO: for loop to count number of john values
    for user in &user_vec {
        if (*user).trim().to_lowercase() == "john".to_owned() {
            counter += 1;
        }
    }

    println!("{} johns in {:?}", counter, user_vec);
}
