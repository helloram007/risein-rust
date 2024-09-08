fn main(){

    // Requirement 6 : Create two BankAccount instances in the main function with different account numbers and holder names.
    let mut bankaccount1 = BankAccount { account_number: 111, holder_name: "Simon".to_string(), balance: 100.0 };
    let mut bankaccount2 = BankAccount { account_number: 222, holder_name: "Ram".to_string(), balance: 200.0 };

    // Requirement 7: Call the deposit method on one of the accounts, passing in a deposit amount.
    println!("Before: Balance A/c 1 {}", bankaccount1.balance);
    match bankaccount1.deposit(20.0) {
        Ok(()) => println!("Deposit successful into account {}", bankaccount1.account_number),
        Err(err) => println!("Failed to deposit into account {}: {}",bankaccount1.account_number, err),
    }

    match bankaccount1.deposit(0.0) {
        Ok(()) => println!("Deposit successful into account {}", bankaccount1.account_number),
        Err(err) => println!("Failed to deposit into account {}: {}",bankaccount1.account_number, err),
    }
    // Requirement 8: Call the withdraw method on the other account, passing in a withdraw amount.
    println!("Before: Balance A/c 2{}", bankaccount2.balance);
    match bankaccount2.withdraw(20.0) {
        Ok(()) => println!("Withdraw successful from account {}", bankaccount2.account_number),
        Err(err) => println!("Failed to withdraw from account {}: {}",bankaccount2.account_number, err),
    }

    // Requirement 9 : Call the balance method on both accounts and print the result to the console.
    println!("After: Balance A/c 1 {}", bankaccount1.balance);
    println!("After: Balance A/c 2 {}", bankaccount2.balance);

}

trait Account {

    fn deposit(&mut self, amount: f64) -> Result<(), String>;
    fn withdraw(&mut self, amount: f64) -> Result<(), String>;
    fn balance(&mut self) -> f64;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl Account for BankAccount {

    // Requirement 3:  In the implementation of the deposit method for BankAccount, add the deposit amount to the balance.
    fn deposit(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Error Message".to_string());
        }
        self.balance += amount;
        Ok(())
    }

    // Requirement 4: In the implementation of the withdraw method for BankAccount, subtract the withdraw amount from the balance.
    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        if amount <= 0.0 {
            return Err("Error Message".to_string());
        }
        self.balance -= amount;
        Ok(())
    }

    // Requirement 5:  In the implementation of the balance method for BankAccount, return the current balance.
    fn balance(&mut self) -> f64 {
        self.balance
    }
}