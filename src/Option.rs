fn sum_of_numbers(numbers: &[u32]) -> Option<u32> {
    let mut sum = 0;

    for &number in numbers {
        match sum.checked_add(number) {
            Some(new_sum) => sum = new_sum,
            None => return None, 
        }
    }

    Some(sum)
}

fn main() {
    let numbers = &[1, 2, 3, 4, 5];
    match sum_of_numbers(numbers) {
        Some(sum) => println!("集合的和: {}", sum),
        None => println!("溢出"),
    }
}
