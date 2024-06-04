//A palindromic number reads the same both ways.
//The largest palindrome made from the product of two 2 digit numbers is 9009 = 91 * 99.
//Find the largest palindrome made from the product of two 3 digit numbers.
fn main() {
    println!("Project Euler #4");

    let mut largest_palin = 0;

    for i in 100..1000 {
        for j in 10..1000 {
            let prod = i * j;
            if is_palindrome(prod) {
                if prod > largest_palin {
                    largest_palin = prod;
                }
            }
        }
    }

    println!("largest palindrome: {}", largest_palin);
}
fn is_palindrome(number: u32) -> bool {
    let s = number.to_string();
    let rev_s: String = s.chars().rev().collect();
    s == rev_s
}
