pub fn nth(n: u32) -> u32 {
    // Declare values needed to track progress in finding nth prime
    let mut prime_counter = 0;
    let mut current_prime = 2;
    let mut modifier = 1;
    // Keep finding primes until we reach nth prime
    while prime_counter != n {
        if is_prime(current_prime + modifier) {
            current_prime = current_prime + modifier;
            prime_counter += 1;
            modifier = 1;
        }
        modifier += 1;
    }
    // Return current_prime since we have found the nth prime
    current_prime
}

pub fn is_prime(n: u32) -> bool {
    // Exit early if n is less then 2 (first prime)
    if n < 2 {
        return false;
    }
    // Test for prime
    !(2..n - 1).any(|i| n % i == 0)
}
