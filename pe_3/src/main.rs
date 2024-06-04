//The prime factors of 13195 are 5, 7, 13 and 29.
//What is the largest prime factor of the number 600851475143

fn main() {
    println!("Project Euler #3");
    let number: u64 = 600851475143;
    let largest_prime = largest_prime_factor(number);
    println!("Largest prime factor of {} is {}", number, largest_prime);
}

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut largest_factor = 1;
    
    while n % 2 == 0 {
        largest_factor = 2;
        n /= 2;
    }
    let mut factor = 3;
    while factor * factor <= n {
        while n % factor == 0 {
            largest_factor = factor;
            n /= factor;
        }
        factor += 2;
    }
    if n > 2 {
        largest_factor = n;
    }
    
    largest_factor
}