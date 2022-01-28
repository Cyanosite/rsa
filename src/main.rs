use rand::Rng;
use std::sync::mpsc;
const MAGIC_NUMBER: i128 = 65537;

fn main() {
    let (tx, rx) = mpsc::channel();
    let mut primes = [0i128; 2];

    std::thread::spawn(move || loop {
        let prime: i128 = rand::thread_rng().gen::<u64>() as i128 % 1000000000;
        if prime_check(prime) {
            println!("{} is a prime", prime);
            tx.send(prime).unwrap();
            break;
        }
    });

    loop {
        let prime: i128 = rand::thread_rng().gen::<u64>() as i128 % 1000000000;
        if prime_check(prime) {
            println!("{} is a prime", prime);
            primes[1] = prime;
            break;
        }
    }
    primes[0] = rx.recv().unwrap();

    let n: i128 = primes[0] * primes[1];
    println!("N = {}", n);
    let phi_n: i128 = (primes[0] - 1) * (primes[1] - 1);
    println!("phi(N) = {}", phi_n);

    let mut mymessage = vec!['a' as i128, 'l' as i128, 'm' as i128, 'a' as i128];
    print_message(&mymessage);
    for letter in &mut mymessage {
        *letter = exponentiation(n, *letter, MAGIC_NUMBER);
    }
    print_message(&mymessage);

    let decryption_key: i128 = congruence(MAGIC_NUMBER, 1, phi_n);
    println!("decryption key = {}", decryption_key);

    for letter in &mut mymessage {
        *letter = exponentiation(n, *letter, decryption_key);
    }
    print_message(&mymessage);
}

fn print_message(message: &Vec<i128>) {
    for letter in message {
        print!("{}", *letter as u8 as char);
    }
    println!("");
}

fn euclidean(mut m: i128, mut a: i128) -> i128 {
    if a > m {
        let temp = a;
        a = m;
        m = temp;
    }
    let mut r: i128;
    loop {
        r = m.rem_euclid(a);
        if r == 0 {
            return a;
        }
        m = a;
        a = r;
    }
}

fn exponentiation(m: i128, mut a: i128, mut b: i128) -> i128 {
    let mut c: i128 = 1;
    loop {
        if b & 0x1 == 0x1 {
            c = (c * a).rem_euclid(m);
        }
        b /= 2;
        if b == 0 {
            return if c < 0 { c + m } else { c };
        }
        a = (a * a).rem_euclid(m);
    }
}

fn prime_check(m: i128) -> bool {
    for _k in 0..100 {
        let a: i128 = rand::thread_rng().gen::<i128>() % (m - 1); //random
        if euclidean(m, a) != 1 {
            return false;
        }
        if exponentiation(m, a, m - 1) != 1 {
            return false;
        }
    }
    return true;
}

fn congruence(mut a: i128, mut b: i128, om: i128) -> i128 {
    let mut m: i128 = om;
    let mut p: i128 = 0;
    loop {
        let t: i128 = m / a;
        let r = m.rem_euclid(a);
        if r == 0 {
            return if b < 0 { om + b } else { b };
        }
        let c = (p - t * b).rem_euclid(om);
        m = a;
        a = r;
        p = b;
        b = c;
    }
}
