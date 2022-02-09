mod algorithms;
pub use crate::algorithms::algorithms::congruence;
pub use crate::algorithms::algorithms::euclidean;
pub use crate::algorithms::algorithms::exponentiation;
pub use crate::algorithms::algorithms::prime_check;

use rand::Rng;
use std::sync::mpsc;
const MAGIC_NUMBER: i128 = 65537;

struct Message {
    message: Vec<i128>,
    is_encrypted: bool,
    public_key: i128,
    private_key: (i128, i128),
}

impl Message {
    fn print(&self) {
        for letter in &self.message {
            print!("{}", *letter as u8 as char);
        }
        println!("");
    }
    fn encrypt(&mut self) {
        let (tx, rx) = mpsc::channel();
        let mut primes = [0i128; 2];

        std::thread::spawn(move || loop {
            let prime: i128 = rand::thread_rng().gen::<u64>() as i128 % 1000000000;
            if prime_check(prime) {
                tx.send(prime).unwrap();
                break;
            }
        });

        loop {
            let prime: i128 = rand::thread_rng().gen::<u64>() as i128 % 1000000000;
            if prime_check(prime) {
                primes[1] = prime;
                break;
            }
        }
        primes[0] = rx.recv().unwrap();
        self.public_key = primes[0] * primes[1];
        self.private_key = (primes[0], primes[1]);
        for letter in &mut self.message {
            *letter = exponentiation(self.public_key, *letter, MAGIC_NUMBER);
        }
        self.is_encrypted = true;
    }
    fn decrypt(&mut self) {
        for letter in &mut self.message {
            let decryption_key = congruence(
                MAGIC_NUMBER,
                1,
                (self.private_key.0 - 1) * (self.private_key.1 - 1),
            );
            *letter = exponentiation(self.public_key, *letter, decryption_key);
        }
    }
}

impl From<&str> for Message {
    fn from(temp: &str) -> Self {
        let mut mymessage = Message {
            message: Vec::new(),
            is_encrypted: false,
            public_key: 0,
            private_key: (0, 0),
        };
        for character in temp.chars() {
            mymessage.message.push(character as i128);
        }
        mymessage.is_encrypted = false;
        mymessage
    }
}

fn main() {
    let mut message = Message::from("Nobody will know that I'm a catgirl");
    message.print();
    message.encrypt();
    message.print();
    message.decrypt();
    message.print();
}
