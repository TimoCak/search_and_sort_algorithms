use std::time::Instant;


/*
sort-methods
*/
pub fn bubble_sort(arr: &mut [usize]) -> &[usize]{
    let mut temp;
    for _i in 0..arr.len() {
        for item in 0..arr.len() {
            if item < arr.len()-1 {
              if arr[item] > arr[item+1] {
                  temp = arr[item];
                  arr[item] = arr[item+1];
                  arr[item+1] = temp;
                } 
              }   
            
          }
    }
    
    arr
}

/****FEHLERHAFT
 * first_item in the array does not sort
 * the algorithm isn't clean!
 * for arr[0] needs to do an extra sorting!
 * ****/
pub fn insertion_sort(arr: &mut [usize]) -> &[usize] {
    for i in 0..arr.len() { 
        let mut j: usize = i - 1; 
        let key = arr[i];
       
        while j>0 && arr[j] > key {
            arr[j + 1] = arr[j];
            j = j - 1;
            
        }
        arr[j + 1] = key;
    } 
    arr
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
