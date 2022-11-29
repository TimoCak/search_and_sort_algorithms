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

pub fn interpolation_search() -> isize {
    -1
}