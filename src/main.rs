mod sort_algo;

fn main() {
    /*
    testing!
    */
    let mut arr = [10, 4 , 3 ,4 ,5, 6 ,7, 3, 2];
    //bubbleSort
    println!("Bubble Sort");
    let arr = sort_algo::bubble_sort(&mut arr);
    for i in  arr {
        println!("{i}");
    }

    //insertionSort
    println!("Insertion Sort!");
    let mut arr = [21 ,10, 9, 4 , 3 ,4 ,5, 6 ,7, 3, 2];
    let arr = sort_algo::insertion_sort(&mut arr);
    for i in  arr {
        println!("{i}");
    }
}
