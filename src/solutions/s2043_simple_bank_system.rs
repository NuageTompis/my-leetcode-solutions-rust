struct Bank {
    balance: Vec<i64>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Self { balance }
    }

    fn transfer(&mut self, account1: i32, account2: i32, money: i64) -> bool {
        let account1 = account1 as usize - 1;
        let account2 = account2 as usize - 1;
        if account1 < self.balance.len() && account2 < self.balance.len() {
            if self.balance[account1] < money {
                false
            } else {
                self.balance[account1] -= money;
                self.balance[account2] += money;
                true
            }
        } else {
            false
        }
    }

    fn deposit(&mut self, account: i32, money: i64) -> bool {
        match self.balance.get_mut(account as usize - 1) {
            Some(v) => {
                *v += money;
                true
            }
            None => false,
        }
    }

    fn withdraw(&mut self, account: i32, money: i64) -> bool {
        match self.balance.get_mut(account as usize - 1) {
            Some(v) => {
                if *v < money {
                    false
                } else {
                    *v -= money;
                    true
                }
            }
            None => false,
        }
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let balance: Vec<i64> = vec![10, 100, 20, 50, 30];
        let mut my_class: Bank = Bank::new(balance);
        let account: i32 = 3;
        let money: i64 = 10;
        let res = my_class.withdraw(account, money);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
        let account1: i32 = 5;
        let account2: i32 = 1;
        let money: i64 = 20;
        let res = my_class.transfer(account1, account2, money);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
        let account: i32 = 5;
        let money: i64 = 20;
        let res = my_class.deposit(account, money);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
        let account1: i32 = 3;
        let account2: i32 = 4;
        let money: i64 = 15;
        let res = my_class.transfer(account1, account2, money);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
        let account: i32 = 10;
        let money: i64 = 50;
        let res = my_class.withdraw(account, money);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }
}
