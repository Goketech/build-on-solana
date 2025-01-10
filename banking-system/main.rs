fn main () {
    println!("Welcome to my banking system!");

    let mut account = BankAccount {
        account_number: 1234567890,
        holder_name: "John Doe".to_string(),
        balance: 1000.0,
    };

    let mut account2 = BankAccount {
        account_number: 1234567891,
        holder_name: "Jane Doe".to_string(),
        balance: 2000.0,
    };

    account.deposit(500.0);
    account2.withdraw(1000.0);

    println!("Account balance: {}", account.balance());
    println!("Account 2 balance: {}", account2.balance());
}

trait Account {
    fn deposit(&mut self, amount: f64);
    fn withdraw(&mut self, amount: f64);
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        self.balance -= amount;
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}