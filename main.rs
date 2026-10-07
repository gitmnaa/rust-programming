fn calculate_total(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

fn main() {
    let numbers = [10, 20, 30, 40, 50];

    println!("Numbers: {:?}", numbers);
    println!("Total: {}", calculate_total(&numbers));
}
