fn main() {
    let mut numbers = vec![1, 2, 3];
    let target = 5;
    let result = find_largest(&numbers, target);
    println!("The largest number that is smaller than or equal to {} is: {}", target, result);
}

fn find_largest(numbers: &[i32], target: i32) -> i32 {
    let mut largest = 0;
    for num in numbers.iter() {
        if *num <= target && num > largest {
            largest = *num;
        }
    }
    largest
}
