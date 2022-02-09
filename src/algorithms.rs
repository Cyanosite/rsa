pub mod algorithms {
    pub fn euclidean(mut m: i128, mut a: i128) -> i128 {
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
    pub fn exponentiation(m: i128, mut a: i128, mut b: i128) -> i128 {
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
    pub fn congruence(mut a: i128, mut b: i128, om: i128) -> i128 {
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
    pub fn prime_check(m: i128) -> bool {
        use rand::Rng;
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
}
