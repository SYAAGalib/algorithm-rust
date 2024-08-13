fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    let mut swapped;

    loop {
        swapped = false;

        for i in 1..n {
            if arr[i - 1] > arr[i] {
                arr.swap(i - 1, i);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        n -= 1;
    }
}

fn main() {
    let mut array = [64, 34, 25, 12, 22, 11, 90];
    println!("Original array: {:?}", array);

    bubble_sort(&mut array);

    println!("Sorted array: {:?}", array);
}
