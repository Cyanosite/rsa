pub mod algorithms {
    use num_bigint::{BigInt, RandBigInt, Sign, ToBigInt};
    pub fn euclidean(mut m: BigInt, mut a: BigInt) -> BigInt {
        if a > m {
            let temp = a;
            a = m;
            m = temp;
        }
        let mut r: BigInt;
        loop {
            r = m % a.clone();
            if r == 0i64.to_bigint().unwrap() {
                return a;
            }
            m = a;
            a = r;
        }
    }
    pub fn exponentiation(m: BigInt, mut a: BigInt, mut b: BigInt) -> BigInt {
        let mut c: BigInt = 1i64.to_bigint().unwrap();
        loop {
            if b.clone() % 2i64 == 1i64.to_bigint().unwrap() {
                c = (c * a.clone()) % m.clone();
            }
            b /= 2i64;
            if b == 0i64.to_bigint().unwrap() {
                return if c < 0i64.to_bigint().unwrap() {
                    c + m
                } else {
                    c
                };
            }
            a = (a.clone() * a.clone()) % m.clone();
        }
    }
    pub fn congruence(mut a: BigInt, mut b: BigInt, om: BigInt) -> BigInt {
        let mut m: BigInt = om.clone();
        let mut p = 0i64.to_bigint().unwrap();
        loop {
            let t: BigInt = m.clone() / a.clone();
            let r = m.clone() % a.clone();
            if r == 0i64.to_bigint().unwrap() {
                return if b < 0i64.to_bigint().unwrap() {
                    om.clone() + b
                } else {
                    b
                };
            }
            let c = (p - t * b.clone()) % om.clone();
            m = a;
            a = r;
            p = b.clone();
            b = c;
        }
    }
    pub fn prime_check(m: BigInt) -> bool {
        for _k in 0..100 {
            let low = BigInt::from_bytes_be(Sign::Plus, b"13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084096");
            let high = m.clone() - 1i64;
            let a = rand::thread_rng().gen_bigint_range(&low, &high); //random
            if euclidean(m.clone(), a.clone()) != 1i64.to_bigint().unwrap() {
                return false;
            }
            if exponentiation(m.clone(), a.clone(), m.clone() - 1i64) != 1i64.to_bigint().unwrap() {
                return false;
            }
        }
        return true;
    }
}
