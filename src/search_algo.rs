pub fn linear_search<T: Ord>(arr: &[T], el: T) -> isize {
    for i in 0..arr.len() {
        if arr[i]==el {
            return i.try_into().unwrap();
        }
    }
    -1
}

pub fn binaery_search<T: Ord>(arr: &[T], el: T) -> isize {
    let mut left = 0;
    let mut right = arr.len()-1;

    while left <= right {
        let middle = (left + right) / 2;
        if arr[middle] < el {
            left = middle + 1;
        } else if arr[middle] > el {
            right = middle - 1;
        } else {
            return middle.try_into().unwrap();
        }
    }
    -1
}

pub fn interpolation_search (arr: &[usize], el: usize) -> isize {
    let mut left: usize = 0;
    let mut right: usize = arr.len()-1;
    let mut middle: usize;
    
    while arr[right] != arr[left] 
          && el >= arr[left] 
          && el <= arr[right] 
    {
       middle = left + ((el - arr[left]) * (right - left) / 
                (arr[right] - arr[left])); 
       if arr[middle] < el {
        left = middle + 1;
       } else if el < arr[middle] {
           right = middle;
       } else {
        return middle.try_into().unwrap();
       }
    }
    -1
}