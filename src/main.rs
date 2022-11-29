mod sort_algo;
mod search_algo;
mod tests;
mod runtime;
fn main() {
    
    let mut arr = [10, 4 , 3 ,4 ,5, 6 ,7, 3, 2];
    //bubbleSort
    println!("Bubble Sort");
    sort_algo::bubble_sort(&mut arr);
    for i in  arr {
        println!("{i}");
    }
    //insertionSort
    println!("Insertion Sort!");
    let mut arr = [21 ,10, 9, 4 , 3 ,4 ,5, 6 ,7, 3, 2];
    sort_algo::insertion_sort(&mut arr);
    for i in  arr {
        println!("{i}");
    }
    println!("Linear Search: {}", search_algo::linear_search(&arr, 4));

    println!("Binaery Search: {}", search_algo::binaery_search(&arr, 4));

    println!("Interpolation Search: {}", search_algo::interpolation_search(&arr, 4));

    println!("RUNTIME BUBBLESORT: {}ms",runtime::calc_bubblesort_runtime());

    println!("RUNTIME INSERTIONSORT: {}ms",runtime::calc_insertionsort_runtime());

    println!("RUNTIME SELECTIONSORT: {}ms",runtime::calc_selectionsort_runtime());

}
