// Rust program to check whether a given number is an ugly number or not
fn main() {
    let mut num = 0; // Taking input from the user
    println!("Enter a number: ");
    num = std::io::stdin().read_line()
        .expect("Failed to read line");
    num = num.trim();
    let num: i32 = num.parse()
        .expect("Please enter a valid integer.");
    if is_ugly(num) {
        println!("{} is an ugly number.", num);
    } else {
        println!("{} is not an ugly number.", num);
    }
}
// Function to check whether the given number is an ugly number or not
fn is_ugly(mut n: i32) -> bool {
    if n == 0 {
        return false;
    }
    while n % 2 == 0 || n % 3 == 0 || n % 5 == 0 {
        if n % 2 == 0 {
            n /= 2;
        } else if n % 3 == 0 {
            n /= 3;
        } else if n % 5 == 0 {
            n /= 5;
        } else {
            return false;
        }
    }
    return true;
}
