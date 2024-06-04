//By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13,
//we can see that the 6th prime is 13.
//What is the 10001st prime number?

fn is_prime(num: u64)->bool{
    if num<2{
        return false;
    }
    
    let sqrt_num=(num as f64).sqrt() as u64;
    for i in 2..=sqrt_num{
        if num%i==0{
            return false;
        }
    }
    return true;
}


fn main() {
    print!("Project Euler #7");
    
    let mut primes=Vec::new();

    let mut num=2;
    while primes.len()<10001 {
        if is_prime(num) {
            primes.push(num);
        }
        num+=1;
    }
    println!("10001st prime number is {}",primes.last().unwrap());
}
