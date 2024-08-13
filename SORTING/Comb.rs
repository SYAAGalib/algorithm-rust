fn comb_sort(arr: &mut [i32]) {
    let mut gap = arr.len();
    let shrink = 1.3;
    let mut sorted = false;

    while gap > 1 || !sorted {
        gap = (gap as f64 / shrink) as usize;
        if gap < 1 {
            gap = 1;
        }

        let mut i = 0;
        sorted = true;

        while i + gap < arr.len() {
            if arr[i] > arr[i + gap] {
                arr.swap(i, i + gap);
                sorted = false;
            }
            i += 1;
        }
    }
}

fn main() {
    let mut arr = [5, 3, 2, 4, 1];
    comb_sort(&mut arr);
    println!("{:?}", arr);
}
