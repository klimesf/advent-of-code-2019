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
