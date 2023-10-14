fn binary_search<T: Ord>(arr: &[T], target: &[T]) ->Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if &arr[mid] < target {
            left = mid + 1;
        } else if &arr[mid] > target {
            right = mid
        } else {
            return Some(mid);
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 7;

    match binary_search(&arr, &target) {
        Some(index) => println!("Element {} found at index {}", target, index),
        None => println!("Element {} not found in the array", target),
    }
}

