pub fn linear_search<T: Ord>(arr: &[T], el: T) -> isize {
    for i in 0..arr.len() {
        if arr[i]==el {
            return i.try_into().unwrap();
        }
    }
    -1
}

pub fn binaery_search<T: Ord>(arr: &[T], el: &T) -> usize {
    arr.len()
}