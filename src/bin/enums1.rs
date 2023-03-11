enum PaymentType {
    CreditCard,
    DebitCard,
    DigitalWallet,
    DirectDebit,
}
impl PaymentType {
    fn description(&self) {
        match self {
            PaymentType::CreditCard => println!("Credit Card payment"),
            PaymentType::DebitCard => println!("Debit Card payment"),
            PaymentType::DigitalWallet => println!("Digital Wallet payment"),
            PaymentType::DirectDebit => println!("Direct Debit payment"),
        }
    }
}
fn main() {
    let payment = PaymentType::DigitalWallet;
    payment.description(); // prints 'Digital Wallet payment'
}
