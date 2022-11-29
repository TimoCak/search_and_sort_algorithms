use std::time::Instant;


/*
sort-methods
*/
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for _i in 0..arr.len() {
        for item in 0..arr.len() {
            if item < arr.len()-1 {
              if arr[item] > arr[item+1] {
                  arr.swap(item, item + 1);
                } 
              }   
            
          }
    }   
}

pub fn insertion_sort<T: Copy + std::cmp::PartialOrd>(arr: &mut [T]) {
    for i in 1..arr.len() { 
        let mut j: usize = i - 1; 
        let key: T = arr[i];
       
        while j>0 && arr[j] > key {
            arr[j + 1] = arr[j];
            j = j - 1;
        }
        arr[j + 1] = key;
    } 
    //sort arr[0] in list
    let mut counter = 0;
    for i in 1..arr.len() {
        if arr[counter] > arr[i] {
           arr.swap(i, counter);
            counter = counter + 1;
        } else {
            break;
        }
    }
}

/*
calc_runtime - methods
*/

pub const ARRAY_SIZE: usize = 10000;

pub fn calc_bubblesort_runtime() -> u128 {
    let mut arr  = [0; ARRAY_SIZE];
    let now = Instant::now();
    bubble_sort(&mut arr);
    let elapsed = now.elapsed();
    return elapsed.as_millis();
}
