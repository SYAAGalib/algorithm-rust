fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge(&mut left, &mut right, arr);
}

fn merge(left: &mut Vec<i32>, right: &mut Vec<i32>, arr: &mut [i32]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut array = [64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", array);

    merge_sort(&mut array);

    println!("Sorted array: {:?}", array);
}
