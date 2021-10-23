use std::cmp::Ordering;

fn main() {
    let array = [3, 17, 75, 80, 202];
    let search_value = 75;
    let bsearch_result = binary_search(&array, search_value);
    match bsearch_result {
        Some(r) => println!("Binary search result at index {}", r),
        None => println!("Value {} not found on array", search_value),
    }
}

fn binary_search(array: &[i32], search_value: i32) -> Option<usize> {
    let mut lower_bound = 0;
    let mut upper_bound = array.len();

    while lower_bound < upper_bound {
        let midpoint = (upper_bound + lower_bound) / 2;
        match search_value.cmp(&array[midpoint]) {
            Ordering::Equal => return Some(midpoint),
            Ordering::Less => upper_bound = midpoint,
            Ordering::Greater => lower_bound = midpoint + 1,
        }
    }
    None
}
