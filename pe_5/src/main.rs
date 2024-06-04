//2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//What is the smallest positive number that is evenly divisible with no remainder by all of the numbers from 1 to 20?

fn main() {
    println!("Project Euler #5");

    let lcm = (2..21).fold(1, |acc, i| acc * i / gcd(i, acc));
    println!("{}", lcm);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    a
}
