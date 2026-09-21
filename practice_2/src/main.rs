use std::collections::HashMap;

struct Account {
    id: u32,
    balance: u64, // in cents
}

struct Bank {
    accounts: HashMap<u32, Account>,
}

impl Bank {
    fn new() -> Bank {
        Bank {
            accounts: HashMap::new(),
        }
    }

    fn open(&mut self, id: u32) {
        self.accounts.insert(id, Account { id, balance: 0 });
    }

    fn balance(&self, id: u32) -> Option<u64> {
        self.accounts.get(&id).map(|acc| acc.balance)
    }

    fn deposit(&mut self, id: u32, amount: u64) -> Result<(), String> {
        if let Some(account) = self.accounts.get_mut(&id) {
            account.balance += amount;
            return Ok(());
        }
        Err("no account matches given id".to_string())
    }

    fn withdraw(&mut self, id: u32, amount: u64) -> Result<(), String> {
        if let Some(account) = self.accounts.get_mut(&id) {
            if account.balance < amount {
                return Err("insufficient funds".to_string());
            }
            account.balance -= amount;
            return Ok(());
        }
        Err("no account matches given id".to_string())
    }

    fn transfer(&mut self, from: u32, to: u32, amount: u64) -> Result<(), String> {
        if from == to {
            return Err("ids cannot be the same".to_string());
        }
        let [from_acc, to_acc] = self.accounts.get_disjoint_mut([&from, &to]);

        let from_acc = from_acc.ok_or("no source account".to_string())?;
        let to_acc = to_acc.ok_or("no destination account".to_string())?;

        if from_acc.balance < amount {
            return Err("insufficient funds".to_string());
        }

        from_acc.balance -= amount;
        to_acc.balance += amount;

        Ok(())
    }
}

fn main() {
    let mut bank = Bank::new();
    bank.open(1);
    bank.open(2);
    bank.deposit(1, 1000).unwrap();

    match bank.transfer(1, 2, 300) {
        Ok(()) => println!("transfer ok"),
        Err(e) => println!("transfer failed: {e}"),
    }
    match bank.withdraw(1, 999999) {
        Ok(()) => println!("withdrew"),
        Err(e) => println!("withdraw failed: {e}"),
    }

    println!("account 1: {:?}", bank.balance(1));
    println!("account 99: {:?}", bank.balance(99));
}
