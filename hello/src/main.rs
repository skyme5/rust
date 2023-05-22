fn main() {
    let m: u64 = gcd(5, 11);
    println!("{}", m);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    debug_assert!(n != 0 && m != 0);

    while m != 0 {
        println!("{} {}", m, n);
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }

    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(11, 5), 1);
    assert_eq!(gcd(12, 4), 4);
}
