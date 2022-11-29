use std::time::Instant;
use rand::Rng;

pub const ARRAY_SIZE: usize = 50000;

pub fn calc_bubblesort_runtime() -> u128 {
    use crate::sort_algo::bubble_sort;
    let mut arr = [0; ARRAY_SIZE];
    fill_array_with_random_numbers(&mut arr);

    let now = Instant::now();
    bubble_sort(&mut arr);
    let elapsed = now.elapsed();
    return elapsed.as_millis();
}

pub fn calc_insertionsort_runtime() -> u128 {
    use crate::sort_algo::insertion_sort;
    let mut arr = [0; ARRAY_SIZE];
    fill_array_with_random_numbers(&mut arr);
    
    let now = Instant::now();
    insertion_sort(&mut arr);
    let elapsed = now.elapsed();
    return elapsed.as_millis();
}

pub fn calc_selectionsort_runtime() -> u128 {
    use crate::sort_algo::selection_sort;
    let mut arr = [0; ARRAY_SIZE];
    fill_array_with_random_numbers(&mut arr);

    let now = Instant::now();
    selection_sort(&mut arr);
    let elapsed = now.elapsed();
    return  elapsed.as_millis();
}

fn fill_array_with_random_numbers(arr: &mut [usize]) {
    for i in 0..arr.len() {
        let random_number = rand::thread_rng().gen_range(1..=ARRAY_SIZE);
        arr[i] = random_number;
    }   
}