use std::vec::Vec;
use std::clone::Clone;

pub trait Sort {
    fn sort<T: Ord + Clone>(&self, array: Vec<T>) -> Vec<T>;
}


pub struct BubbleSort;
impl Sort for BubbleSort {
    fn sort<T: Ord>(&self, mut array: Vec<T>) -> Vec<T> {
        let len = array.len();
        for i in 0..len {
            for j in 0..len - 1 - i {
                if array[j] > array[j + 1] {
                    array.swap(j, j + 1);
                }
            }
        }
        array
    }
}


pub struct QuickSort;
impl Sort for QuickSort {
    fn sort<T: Ord>(&self, mut array: Vec<T>) -> Vec<T> {
        let len = array.len();
        self.sort_impl(&mut array, 0, len - 1);
        array
    }
}

impl QuickSort {
    fn partition<T: Ord>(&self, array: &mut Vec<T>, left: usize, right: usize) -> usize {
        let len = right - left + 1;
        if len <= 1 {
            return right;
        }

        let mut pivot = (left + right) / 2;
        let mut i = left;
        let mut j = right;

        /* for median-of-3 */
        if array[i] > array[pivot] {
            array.swap(i, pivot);
        }
        if array[j] < array[pivot] {
            array.swap(j, pivot);
            if array[i] > array[pivot] {
                array.swap(i, pivot);
            }
        }

        loop {
            while array[i] < array[pivot] {
                i += 1;
            }
            while array[j] > array[pivot] {
                j -= 1;
            }
            if i >= j {
                break;
            }

            array.swap(i, j);
            if pivot == i {
                pivot = j;
            } else if pivot == j {
                pivot = i;
            }

            i += 1;
            j -= 1;
        }

        j + 1
    }

    fn sort_impl<T: Ord>(&self, array: &mut Vec<T>, left: usize, right: usize) {
        let len = right - left + 1;
        if len <= 1 {
            return;
        }

        let index = self.partition(array, left, right);
        self.sort_impl(array, left, index - 1);
        self.sort_impl(array, index, right);
    }
}


pub struct MergeSort;
impl Sort for MergeSort {
    fn sort<T: Ord + Clone>(&self, mut array: Vec<T>) -> Vec<T> {
        let len = array.len();
        let mut indices: Vec<usize> = vec![0; len];
        self.sort_impl(&mut array, &mut indices, 0, len);
        array
    }
}

impl MergeSort {
    fn sort_impl<T: Ord + Clone>(&self, array: &mut Vec<T>, indices: &mut Vec<usize>, left: usize, right: usize) {
        if left == right || left == right - 1 {
            return;
        }

        let mid = (left + right) / 2;
        self.sort_impl(array, indices, left, mid);
        self.sort_impl(array, indices, mid, right);
        self.merge(array, indices, left, mid, right);
    }

    fn merge<T: Ord + Clone>(&self, array: &mut Vec<T>, indices: &mut Vec<usize>, left: usize, mid: usize, right: usize) {
        let data: Vec<T> = array.clone();

        let mut i = left;
        let mut j = mid;
        let mut k = 0;

        /* compare left-side with right-side */
        while i < mid && j < right {
            if array[i] <= array[j] {
                indices[k] = i;
                i += 1;
            } else {
                indices[k] = j;
                j += 1;
            }
            k += 1;
        }
        /* fill with right-side */
        if i == mid {
            while j < right {
                indices[k] = j;
                k += 1;
                j += 1;
            }
        /* fill with left-side */
        } else {
            while i < mid {
                indices[k] = i;
                k += 1;
                i += 1;
            }
        }

        for l in 0..k {
            array[left + l] = data[indices[l]].clone();
        }
    }
}
