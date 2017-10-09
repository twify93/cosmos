use std::collections::HashMap;

fn main() {
    // 2 results - for `7` and both `-3` numbers
    two_sum(vec![3, 5, 7, 0, -3, -2, -3], 4);
    // No results
    two_sum(vec![3, 5, 0, -3, -2, -3], 4);
}

fn two_sum(numbers: Vec<i32>, target: i32) {
    let mut indices: HashMap<i32, usize> = HashMap::new();

    println!("Numbers: {:?}", numbers);

    for (i, number) in numbers.iter().enumerate() {
        let hash_index = target - number;

        let has_number = match indices.get(&hash_index) {
            Some(_) => true,
            None => false,
        };
        if has_number {
            println!("Found two indices that sum-up to {} are : {} and {}", target, indices.get(&hash_index).unwrap(), i);
        }
        indices.insert(*number, i);
    }
}