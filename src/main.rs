mod algorithms;
pub use crate::algorithms::algorithms::congruence;
pub use crate::algorithms::algorithms::euclidean;
pub use crate::algorithms::algorithms::exponentiation;
pub use crate::algorithms::algorithms::prime_check;

use num_bigint::{BigInt, RandBigInt, Sign, ToBigInt};
use std::sync::mpsc;

struct Message {
    message: Vec<BigInt>,
    is_encrypted: bool,
    public_key: BigInt,
    private_key: (BigInt, BigInt),
}

impl Message {
    fn print(&self) {
        for letter in &self.message {
            print!(
                "{}",
                *letter.to_signed_bytes_le().first().unwrap() as u8 as char
            );
        }
        println!("");
    }
    fn encrypt(&mut self) {
        let (tx, rx) = mpsc::channel();
        let mut primes = [
            BigInt::from_bytes_be(Sign::Plus, b"0"),
            BigInt::from_bytes_be(Sign::Plus, b"0"),
        ];

        let low = BigInt::from_bytes_be(Sign::Plus, b"13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084096");
        let high = BigInt::from_bytes_be(Sign::Plus, b"179769313486231590772930519078902473361797697894230657273430081157732675805500963132708477322407536021120113879871393357658789768814416622492847430639474124377767893424865485276302219601246094119453082952085005768838150682342462881473913110540827237163350510684586298239947245938479716304835356329624224137216");
        std::thread::spawn(move || loop {
            let prime = rand::thread_rng().gen_bigint_range(&low, &high);
            if prime_check(prime.clone()) {
                println!("{} is a prime", prime);
                tx.send(prime).unwrap();
                break;
            }
        });
        let low = BigInt::from_bytes_be(Sign::Plus, b"13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084096");
        let high = BigInt::from_bytes_be(Sign::Plus, b"179769313486231590772930519078902473361797697894230657273430081157732675805500963132708477322407536021120113879871393357658789768814416622492847430639474124377767893424865485276302219601246094119453082952085005768838150682342462881473913110540827237163350510684586298239947245938479716304835356329624224137216");
        loop {
            let prime = rand::thread_rng().gen_bigint_range(&low, &high);
            if prime_check(prime.clone()) {
                println!("{} is a prime", prime);
                primes[1] = prime;
                break;
            }
        }
        primes[0] = rx.recv().unwrap();
        self.public_key = primes[0].clone() * primes[1].clone();
        self.private_key = (primes[0].clone(), primes[1].clone());
        for letter in &mut self.message {
            *letter = exponentiation(
                self.public_key.clone(),
                letter.clone(),
                65537i64.to_bigint().unwrap(),
            );
        }
        self.is_encrypted = true;
    }
    fn decrypt(&mut self) {
        for letter in &mut self.message {
            let decryption_key = congruence(
                65537i64.to_bigint().unwrap(),
                1i64.to_bigint().unwrap(),
                (self.private_key.clone().0 - 1i64) * (self.private_key.clone().1 - 1i64),
            );
            *letter = exponentiation(self.public_key.clone(), letter.clone(), decryption_key);
        }
    }
}

impl From<&str> for Message {
    fn from(temp: &str) -> Self {
        let mut mymessage = Message {
            message: Vec::new(),
            is_encrypted: false,
            public_key: 0i64.to_bigint().unwrap(),
            private_key: (0i64.to_bigint().unwrap(), 0i64.to_bigint().unwrap()),
        };
        for character in temp.chars() {
            mymessage
                .message
                .push((character as i64).to_bigint().unwrap());
        }
        mymessage.is_encrypted = false;
        mymessage
    }
}

fn main() {
    let mut message = Message::from("Ezt az üzenetet senki sem láthatja!!!");
    message.print();
    message.encrypt();
    message.print();
    message.decrypt();
    message.print();
}
