use crate::sort_algo::bubble_sort;

mod sort_algo;
mod search_algo;
fn main() {
    /*
    testing!
    */
    let mut arr = [10, 4 , 3 ,4 ,5, 6 ,7, 3, 2];
    //bubbleSort
    println!("Bubble Sort");
    sort_algo::bubble_sort(&mut arr);
    for i in  arr {
        println!("{i}");
    }
    //runtime
    println!("{}ms",sort_algo::calc_bubblesort_runtime());

    //insertionSort
    println!("Insertion Sort!");
    let mut arr = [21 ,10, 9, 4 , 3 ,4 ,5, 6 ,7, 3, 2];
    sort_algo::insertion_sort(&mut arr);
    for i in  arr {
        println!("{i}");
    }
    //runtime
    println!("{}ms",sort_algo::calc_insertionsort_runtime());

    println!("Linear Search: {}", search_algo::linear_search(&arr, 9));

    print!("Binaery Search: {}", search_algo::binaery_search(&arr, 9));

}
