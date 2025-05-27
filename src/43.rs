fn main() {
    let mut num1 = 5;
    let num2 = 3;

    // Addition
    println!("The sum is: {}", num1 + num2);

    // Subtraction
    println!("The difference is: {}", num1 - num2);

    // Multiplication
    println!("The product is: {}", num1 * num2);

    // Division
    if num2 != 0 {
        println!("The quotient is: {}", num1 / num2);
    } else {
        println!("Division by zero is not allowed.");
    }

    // Exponentiation
    let exponent = 4;
    println!("4 raised to the power of 3 is: {}", pow(exponent, 3));

    // Modulus operation
    let modulus_result = 10 % 5;
    println!("The result of 10 % 5 is: {}", modulus_result);
}

fn pow(base: i32, exponent: i32) -> i32 {
    if base == 0 {
        panic!("Division by zero is not allowed.");
    }
    let mut result = 1;
    while exponent != 0 {
        if exponent % 2 == 1 {
            result *= base;
        }
        base *= base;
        exponent /= 2;
    }
    result
}
