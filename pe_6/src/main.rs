//The sum of the squares of the first ten natural numbers is,
//1^2 + 2^2 + ... + 10^2 = 385
//The square of the sum of the first ten natural numbers is,
//(1 + 2 + ... + 10)^2 = 55^2 = 3025.
//Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
//Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {
    println!("Project Euler #6");
    let mut square_sum = 0;
    let mut sum_square = 0;
    for i in 1..101 {
        square_sum += i * i;
        sum_square += i;
    }

    println!("Squared sum: {}", square_sum);
    // println!("Sum squared: {}", sum_square);

    sum_square = sum_square * sum_square;

    println!("Sum squared: {}", sum_square);

    let res = sum_square - square_sum;

    println!("Sum of squares: {}", res);
}
