// 11. Tim Sort
const MIN_MERGE: usize = 32;

fn min_run_length(n: usize) -> usize {
    let mut r = 0;
    let mut n = n;
    while n >= MIN_MERGE {
        r |= n & 1;
        n >>= 1;
    }
    n + r
}

fn insertion_sort(arr: &mut [i32], left: usize, right: usize) {
    for i in left + 1..=right {
        let key = arr[i];
        let mut j = i;
        while j > left && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn merge(arr: &mut [i32], left: usize, mid: usize, right: usize) {
    let len1 = mid - left + 1;
    let len2 = right - mid;
    let mut left_arr = Vec::with_capacity(len1);
    let mut right_arr = Vec::with_capacity(len2);

    for i in 0..len1 {
        left_arr.push(arr[left + i]);
    }
    for j in 0..len2 {
        right_arr.push(arr[mid + 1 + j]);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    while i < len1 && j < len2 {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < len1 {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < len2 {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

fn tim_sort(arr: &mut [i32]) {
    let n = arr.len();
    let min_run = min_run_length(MIN_MERGE);

    for i in (0..n).step_by(min_run) {
        insertion_sort(arr, i, ((i + min_run - 1).min(n - 1)));
    }

    let mut size = min_run;
    while size < n {
        for left in (0..n).step_by(2 * size) {
            let mid = (left + size - 1).min(n - 1);
            let right = ((left + 2 * size - 1).min(n - 1));
            if mid < right {
                merge(arr, left, mid, right);
            }
        }
        size *= 2;
    }
}

fn main() {
    let mut arr = [5, 21, 7, 23, 19];
    tim_sort(&mut arr);
    println!("{:?}", arr);
}
