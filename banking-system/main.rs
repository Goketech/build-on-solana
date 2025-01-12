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

    match account.deposit(500.0) {
        Ok(()) => println!("Deposit successful!"),
        Err(err) => println!("Failed to deposit: {}", err),
    };

    match account2.withdraw(1000.0) {
        Ok(()) => println!("Withdrawal successful!"),
        Err(err) => println!("Failed to withdraw: {}", err),
    };

    println!("Account balance: {}", account.balance());
    println!("Account 2 balance: {}", account2.balance());

    match account.deposit(-500.0) {
        Ok(()) => println!("Deposit successful!"),
        Err(err) => println!("Failed to deposit: {}", err),
    }

    match account2.withdraw(2500.0) {
        Ok(()) => println!("Withdrawal successful!"),
        Err(err) => println!("Failed to withdraw: {}", err),
    }

    println!("Account balance: {}", account.balance());
    println!("Account 2 balance: {}", account2.balance());
}

trait Account {
    fn deposit(&mut self, amount: f64)  -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount < 0.0 {
            return Err("Can't deposit negative amount".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount > self.balance {
            return Err("Insufficient funds".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}