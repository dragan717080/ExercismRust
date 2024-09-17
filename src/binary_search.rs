fn main() {
    assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
}

pub fn find(arr: &[i32], v: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len();
    let mut mid;

    // Empty arrays or values that are not part within array value bounds are not included
    if arr.len() == 0 || arr[0] > v || *arr.last().unwrap() < v {
        return None;
    }

    while low <= high {
        mid = low + (high - low) / 2;

        if arr[mid] == v {
            return Some(mid);
        } else if arr[mid] < v {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}
