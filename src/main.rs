use chrono::{DateTime, Utc};
use std::fmt;

#[derive(PartialEq, Debug)]
struct AccountStatementLine {
    date: DateTime<Utc>,
    amount: isize,
    balance: isize
}

impl AccountStatementLine {
    fn header() -> String {
        format!("{:^30} | {:>10} | {:>10}", "Date", "Amount", "Balance")
    }
}

impl fmt::Display for AccountStatementLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:^30} | {:>10} | {:>10}", self.date.format("%Y-%m-%d %H:%M:%S%.f").to_string(), self.amount, self.balance)
    }
}

#[derive(PartialEq, Debug)]
struct AccountStatement {
    lines: Vec<AccountStatementLine>
}
// would love to iterate on this one directly

#[derive(PartialEq, Debug)]
enum OperationType {
    Withdraw,
    Deposit,
}

#[derive(PartialEq, Debug)]
struct Money(usize);

#[derive(PartialEq, Debug)]
struct Operation {
    operation_type: OperationType,
    amount: Money,
    date: DateTime<Utc>
}

impl Operation {
    fn value(&self) -> isize {
        match self.operation_type {
            OperationType::Deposit => isize::try_from(self.amount.0).unwrap(),
            OperationType::Withdraw => -isize::try_from(self.amount.0).unwrap(),
        }
    }

}

#[derive(PartialEq, Debug)]
struct BankAccount {
    operations: Vec<Operation>
}

impl BankAccount {
    fn new() -> BankAccount {
        BankAccount {
            operations: vec![]
        }
    }

    fn balance(&self) -> isize {
        self.operations.iter().map(|op| op.value()).sum()
    }

    fn make_deposit(&mut self, money: Money, date: DateTime<Utc>) {
        self.operations.push(Operation {
            operation_type: OperationType::Deposit,
            amount: money,
            date,
        })
    }
    
    fn make_withdrawal(&mut self, money: Money, date: DateTime<Utc>) -> Result<(), &str> {
        let new_balance = self.balance() - isize::try_from(money.0).unwrap();
        if new_balance < -50 {
            Err("Not enough money")
        } else {
            self.operations.push(Operation {
                operation_type: OperationType::Withdraw,
                amount: money,
                date,
            });
            Ok(())
        }
    }

    fn to_statement(&self) -> AccountStatement {
        let mut balance = 0;
        let mut lines: Vec<AccountStatementLine> = self.operations.iter().map(|op| {
                balance += op.value();
                AccountStatementLine {
                    date: op.date,
                    amount: op.value(),
                    balance
                }
            }
        ).collect();
        lines.sort_by(|a, b| b.date.cmp(&a.date));
        AccountStatement { lines }
    }

    fn print_statement(&self) {
        println!("{}", AccountStatementLine::header());
        for statement in self.to_statement().lines {
            println!("{}", statement)
        }
    }
}

fn main() {
    let mut account = BankAccount::new();
    account.make_deposit(Money(100), Utc::now());
    account.make_withdrawal(Money(20), Utc::now()).map_err(|_| println!("Not enough money")).unwrap_or(());
    account.make_withdrawal(Money(200), Utc::now()).map_err(|_|println!("Not enough money")).unwrap_or(());
    account.make_deposit(Money(50), Utc::now());
    account.make_withdrawal(Money(120), Utc::now()).map_err(|_|println!("Not enough money")).unwrap_or(());
    account.print_statement()
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_create_account() {

        let account = BankAccount::new();
        assert_eq!(account.balance(), 0);
    }

    #[test]
    fn test_make_deposit() {
        let mut account = BankAccount::new();
        account.make_deposit(Money(100), Utc::now());
        assert_eq!(account.balance(), 100);
    }

    #[test]
    fn test_make_withdrawal() {
        let mut account = BankAccount::new();
        account.make_withdrawal(Money(50), Utc::now()).unwrap();
        assert_eq!(account.balance(), -50);
    }

    #[test]
    fn test_withdrawal_refused_if_balance_falls_below_50() {
        let mut account = BankAccount::new();
        let withdrawal = account.make_withdrawal(Money(51), Utc::now());
        assert_eq!(withdrawal, Err("Not enough money"));
    }

    #[test]
    fn test_account_statement() {
        let mut account = BankAccount::new();
        account.make_deposit(Money(10), Utc.ymd(2022, 1, 14).and_hms(8, 9, 10));
        account.make_deposit(Money(20), Utc.ymd(2022, 1, 15).and_hms(8, 9, 10));
        account.make_withdrawal(Money(15), Utc.ymd(2022, 1, 18).and_hms(8, 9, 10)).unwrap();
        let lines = vec![
            AccountStatementLine {
                date: Utc.ymd(2022, 1, 18).and_hms(8, 9, 10),
                amount: -15,
                balance: 15
            },
            AccountStatementLine {
                date: Utc.ymd(2022, 1, 15).and_hms(8, 9, 10),
                amount: 20,
                balance: 30
            },
            AccountStatementLine {
                date: Utc.ymd(2022, 1, 14).and_hms(8, 9, 10),
                amount: 10,
                balance: 10
            }
        ];
        assert_eq!(account.to_statement(), AccountStatement {
            lines
        });
    }
}
