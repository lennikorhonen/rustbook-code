// Given a list of integers, use a vector and return the median (when sorted, the 
// value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here)
// of the list.
use std::collections::HashMap;

fn main() {
    // let list_of_ints: Vec<i32> = Vec::new();
    let list_of_ints = vec![4, 5, 9, 2, 4, 12, 42, 42, 42, 42, 42, 42];

    let list_median = int_list_median(list_of_ints.clone());
    println!("List median {:?}", list_median);

    let list_mode = occurs_most_often(list_of_ints);
    println!("{:?}", list_mode);
}

fn int_list_median(mut list_of_ints: Vec<i32>) -> i32 {
    list_of_ints.sort();

    let list_mid = list_of_ints.len() / 2;
    println!("List middle index: {:?}", list_mid);
    println!("List {:?}", list_of_ints);
    list_of_ints[list_mid]
}

fn occurs_most_often(list_of_ints: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in list_of_ints {
        let count = map.entry(num).or_insert(0); 
        *count += 1;
    }

    let mut mode_key = 0;
    let mut prev = 0;

    for (key, value) in map {
        if value > prev {
            mode_key = key;
            prev = value;
        } else {
            prev = value;
        }
    }

    mode_key
}
