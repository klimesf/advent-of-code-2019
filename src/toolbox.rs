use core::mem;

pub(crate) fn gcd(mut a: i32, mut b: i32) -> i32 {
    if b > a {
        mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a.max(-a);
}

pub(crate) fn lcm(a: i32, b: i32) -> i32 {
    if a > b {
        (a * b) / gcd(a, b)
    } else {
        (a * b) / gcd(b, a)
    }
}

pub(crate) fn gcd_64(mut a: i64, mut b: i64) -> i64 {
    if b > a {
        mem::swap(&mut a, &mut b);
    }
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a.max(-a);
}

pub(crate) fn lcm_64(a: i64, b: i64) -> i64 {
    if a > b {
        (a * b) / gcd_64(a, b)
    } else {
        (a * b) / gcd_64(b, a)
    }
}
