// 10. Shell Sort
fn shell_sort(arr: &mut [i32]) {
    let n = arr.len();
    let mut gap = n / 2;

    while gap > 0 {
        for i in gap..n {
            let temp = arr[i];
            let mut j = i;

            while j >= gap && arr[j - gap] > temp {
                arr[j] = arr[j - gap];
                j -= gap;
            }
            arr[j] = temp;
        }
        gap /= 2;
    }
}

fn main() {
    let mut arr = [12, 34, 54, 2, 3];
    shell_sort(&mut arr);
    println!("{:?}", arr);
}
