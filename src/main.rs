use bit_vec::BitVec;
fn approx_num_primes_less_than(n: u32) -> u32 {
    // very approximate:
    let x = n as f64;
    let num_primes = x / x.ln();
    let plus_10_percent = num_primes * 1.2;
    println!(
        "Approximate number of primes less than {}: {}",
        n, plus_10_percent
    );
    return plus_10_percent.round() as u32;
}

#[inline]
fn mark_as_prime(n: u32, i: usize, is_prime: &mut BitVec) {
    let mut j = i * 2;
    while j <= n as usize {
        is_prime.set(j, false);
        j += i;
    }
}

fn get_primes_to(n: u32) -> Vec<u32> {
    let sqrt_n = (n as f64).sqrt().ceil() as usize;
    let mut is_prime = BitVec::from_elem(n as usize + 1, true);
    is_prime.set(0, false);
    is_prime.set(1, false);
    is_prime.set(2, true); // maybe this is a bit of a cheat...
    mark_as_prime(n, 2, &mut is_prime);

    for i in (3..=sqrt_n as usize).step_by(2) {
        if is_prime[i] {
            mark_as_prime(n, i, &mut is_prime);
        }
    }

    let mut primes = Vec::with_capacity(approx_num_primes_less_than(n) as usize);

    primes.push(2);
    for (i, b) in is_prime.iter().enumerate().skip(3).step_by(2) {
        if b {
            primes.push(i as u32);
        }
    }

    primes
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_primes() {
        assert_eq!(get_primes_to(10), vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_get_primes_to_25() {
        assert_eq!(get_primes_to(25), vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
    }

    #[test]
    fn test_num_primes_less_than() {
        assert!(approx_num_primes_less_than(10) > 4);
        assert!(approx_num_primes_less_than(100) > 25);
    }
}

fn main() {
    let n = u32::MAX; // takes about 50s on my M1 Pro 14"
    let primes = get_primes_to(n);
    println!("Number of primes less than {}: {}", n, primes.len());
}
