
fn main() {
    let mut account = BankAccount {
        owner :String::from("Alice"),
        balance : 1000.0,
    } ; 
    let current_balance = account.check_balance();
    println!("The First balance was: {}", current_balance);
    account.withdraw(200.0);
    println!("Current balance: {}", account.check_balance());
}

struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn  withdraw(&mut self, amount:f64){
        println!("Withdrawing {} from {}", amount, self.owner);
        self.balance -= amount;
    }
    fn check_balance(&self) -> f64 {
        self.balance
    }
}