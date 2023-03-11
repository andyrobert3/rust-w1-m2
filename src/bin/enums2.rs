enum PaymentType {
    CreditCard { credit_card_number: i32 },
    DirectDebit(i32),
}
fn main() {
    let payment_type = PaymentType::DirectDebit(12341234);

    match payment_type {
        PaymentType::CreditCard { credit_card_number } => {
            println!("Credit Card: {}", credit_card_number)
        }
        PaymentType::DirectDebit(x) => println!("Bank Account: {}", x), // binding on x
        _ => println!("Unknown payment type"),
    }
}
