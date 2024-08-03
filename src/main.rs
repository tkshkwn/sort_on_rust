use std::time::Instant;

mod sort;
use crate::sort::Sort;

fn main() {
    let vec = vec![4, 5, 6, 3, 7, 8, 9, 2, 0, 1];

    let target = vec.clone();
    let start = Instant::now();
    let result = sort::BubbleSort.sort(target);
    let duration = start.elapsed();
    println!("BubbleSort:{:?}, time:{:?}", result, duration);

    let target = vec.clone();
    let start = Instant::now();
    let result = sort::QuickSort.sort(target);
    let duration = start.elapsed();
    println!("QuickSort:{:?}, time:{:?}", result, duration);

    let target = vec.clone();
    let start = Instant::now();
    let result = sort::MergeSort.sort(target);
    let duration = start.elapsed();
    println!("MergeSort:{:?}, time:{:?}", result, duration);

    let target = vec.clone();
    let start = Instant::now();
    let result = sort::SelectionSort.sort(target);
    let duration = start.elapsed();
    println!("SelectionSort:{:?}, time:{:?}", result, duration);

    let target = vec.clone();
    let start = Instant::now();
    let result = sort::InsertionSort.sort(target);
    let duration = start.elapsed();
    println!("InsertionSort:{:?}, time:{:?}", result, duration);
}
