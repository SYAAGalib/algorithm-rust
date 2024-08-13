fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn main() {
    let mut array = [64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", array);

    insertion_sort(&mut array);

    println!("Sorted array: {:?}", array);
}
