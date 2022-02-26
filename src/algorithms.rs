pub mod algorithms {
    use num_bigint::{BigInt, RandBigInt, Sign, ToBigInt};
    pub fn euclidean(mut m: BigInt, mut a: BigInt) -> BigInt {
        let mut r: BigInt;
        let null = 0i64.to_bigint().unwrap();
        loop {
            r = m % &a;
            if r == null {
                return a;
            }
            m = a;
            a = r.clone();
        }
    }
    pub fn exponentiation(m: BigInt, mut a: BigInt, mut b: BigInt) -> BigInt {
        let mut c: BigInt = 1i64.to_bigint().unwrap();
        let null = 0i64.to_bigint().unwrap();
        let one = 1i64.to_bigint().unwrap();
        loop {
            if &b % 2i64 == one {
                c = (c * &a) % &m;
            }
            b = b >> 1;
            if b == null {
                return if c < null { c + m } else { c };
            }
            a = (&a * &a) % &m;
        }
    }
    pub fn congruence(mut a: BigInt, mut b: BigInt, om: BigInt) -> BigInt {
        let mut m: BigInt = om.clone();
        let mut p = 0i64.to_bigint().unwrap();
        let null = 0i64.to_bigint().unwrap();
        loop {
            let t: BigInt = &m / &a;
            let r = &m % &a;
            if r == null {
                return if b < null { om + b } else { b };
            }
            let c = (p - t * &b) % &om;
            m = a;
            a = r;
            p = b.clone();
            b = c;
        }
    }
    pub fn prime_check(m: &BigInt) -> bool {
        let low = BigInt::from_bytes_be(Sign::Plus, b"13407807929942597099574024998205846127479365820592393377723561443721764030073546976801874298166903427690031858186486050853753882811946569946433649006084096");
        let high = m - 1i64;
        let one = 1i64.to_bigint().unwrap();
        for _k in 0..100 {
            let a = rand::thread_rng().gen_bigint_range(&low, &high); //random
            if euclidean(m.clone(), a.clone()) != one {
                return false;
            }
            if exponentiation(m.clone(), a.clone(), m.clone() - 1i64) != one {
                return false;
            }
        }
        true
    }
}
