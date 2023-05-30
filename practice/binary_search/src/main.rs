fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid -1;
        }
    }

    None
}


fn main() {
    let arr = [3, 5, 1, 4, 2];
    let target = 4;

    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    match binary_search(&sorted_arr, target) {
        Some(index) => println!("Target found at index {}", index),
        None => println!("Target not found"),
    }
}
