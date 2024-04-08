pub fn nth(n: u32) -> u32 {
    let mut index: u32 = 0;
    let mut prime: u32 = 2;
    loop {
        let mut is_prime: bool = true;
        for p in 2..prime {
            if prime % p == 0 {
                is_prime = false;
                break;
            }
        }
        if is_prime {
            if n == index {
                return prime;
            }
            index += 1;
        }
        prime += 1;
    }
}
#[test]
fn first_prime() {
    assert_eq!(nth(0), 2);
}
#[test]
fn second_prime() {
    assert_eq!(nth(1), 3);
}
#[test]
fn sixth_prime() {
    assert_eq!(nth(5), 13);
}
#[test]
fn big_prime() {
    assert_eq!(nth(10_000), 104_743);
}