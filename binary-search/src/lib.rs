pub fn find<T: AsRef<[I]>, I: Ord>(array: T, key: I) -> Option<usize> {
    let array = array.as_ref();
    let mut low = 0;
    let mut high = array.len();
    while low < high {
        let mid = low + (high - low) / 2;
        if array[mid] < key {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    if low == array.len() || array[low] != key {
        None
    } else {
        Some(low)
    }
}
