fn cocktail_sort(arr: &mut [i32]) {
    let mut swapped = true;
    let mut start = 0;
    let mut end = arr.len() - 1;

    while swapped {
        swapped = false;

        // Forward pass
        for i in start..end {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        swapped = false;
        end -= 1;

        // Backward pass
        for i in (start..end).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        start += 1;
    }
}

fn main() {
    let mut arr = [5, 1, 4, 2, 8, 0, 2];
    cocktail_sort(&mut arr);
    println!("{:?}", arr);
}
