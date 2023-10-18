use std::fmt;
use std::result::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Coin {
    TwoPound,
    OnePound,
    FiftyPence,
    TwentyPence,
    TenPence,
    FivePence,
}

#[derive(Debug, Clone)]
struct WithdrawError;

impl fmt::Display for WithdrawError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Insufficient funds")
    }
}

struct Bank {
    _coins: HashMap<Coin, u32>,
    _coin_list: [Coin; 6],
    _coin_map: HashMap<Coin, f32>,
}

impl Bank {
    fn new(coins: HashMap<Coin, u32>) -> Bank {
        Self {
            _coins: coins.clone(),
            _coin_list: [
                Coin::TwoPound,
                Coin::OnePound,
                Coin::FiftyPence,
                Coin::TwentyPence,
                Coin::TenPence,
                Coin::FivePence
            ],
            _coin_map: HashMap::from([
                (Coin::TwoPound, 2.0),
                (Coin::OnePound, 1.0),
                (Coin::FiftyPence, 0.5),
                (Coin::TwentyPence, 0.2),
                (Coin::TenPence, 0.1),
                (Coin::FivePence, 0.05),
            ]),
        }
    }

    fn _report(&self) -> f32 {
        let mut r: f32 = 0.0;
        for c in &self._coin_list {
            match self._coins.get(&c) {
                Some(v) => {
                    r += (*v as f32) * self._coin_map.get(&c).unwrap();
                },
                None => {},
            };
        }
        r
    }

    fn report(&self) {
        println!("Available funds: {}", self._report());
    }

    fn deposit(&mut self, deposit: HashMap<Coin, u32>) {
        for c in &self._coin_list {
            match deposit.get(&c) {
                Some(v) => {
                    println!("Depositing {} of {:?}", v, c);
                    let t: &mut u32 = self._coins.get_mut(&c).unwrap();
                    *t += *v;
                },
                None => {},
            }
        }
        self.report()
    }

    fn withdraw(&mut self, withdraw: f32) -> Result<HashMap<Coin, u32>, WithdrawError> {
        if withdraw > self._report() {
            return Result::Err(WithdrawError);
        }
        // solve the coins
        Result::Ok(HashMap::from([(Coin::TwoPound, 0)]))
    }
}

fn main() {
    // Determine how much money is in a piggy bank that contains several £2 coins,
    // £1 coins, 50p coins, 20p coins, 10p coins and 5p coins.
    // Use the following values to test your programme:
    // one £2, three £1, five 50p  coins, two 20p coins, one 10p coin and fifteen 5p coins.
    let mut bank: Bank = Bank::new(
        HashMap::from([
            (Coin::TwoPound, 1),
            (Coin::OnePound, 3),
            (Coin::FiftyPence, 5),
            (Coin::TwentyPence, 2),
            (Coin::TenPence, 1),
            (Coin::FivePence, 15),
        ]),
    );

    bank.report();

    bank.deposit(
        HashMap::from([
            (Coin::TwoPound, 2),
            (Coin::TwentyPence, 8),
            (Coin::FivePence, 3),
        ]),
    );

    bank.deposit(
        HashMap::from([
            (Coin::OnePound, 1),
            (Coin::TenPence, 5),
        ]),
    );

    let w: HashMap<Coin, u32> = match bank.withdraw(300.0) {
        Ok(v) => {v},
        Err(WithdrawError) => {
            println!("You do not have the funds {:?}", WithdrawError);
            HashMap::<Coin, u32>::new()
        },
        Err(_) => panic!("Oh no"),
    };

    bank.report();

    let w: HashMap<Coin, u32> = match bank.withdraw(8.75) {
        Ok(v) => {v},
        Err(WithdrawError) => {
            println!("You do not have the funds {:?}", WithdrawError);
            HashMap::<Coin, u32>::new()
        },
        Err(_) => panic!("Oh no"),
    };

    bank.report();
}
