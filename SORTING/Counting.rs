fn counting_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    let max = *arr.iter().max().unwrap();
    let min = *arr.iter().min().unwrap();
    let range = (max - min + 1) as usize;

    let mut count = vec![0; range];
    let mut output = vec![0; arr.len()];

    for &num in arr.iter() {
        count[(num - min) as usize] += 1;
    }

    for i in 1..range {
        count[i] += count[i - 1];
    }

    for &num in arr.iter().rev() {
        output[count[(num - min) as usize] as usize - 1] = num;
        count[(num - min) as usize] -= 1;
    }

    arr.copy_from_slice(&output);
}

fn main() {
    let mut arr = [4, 2, 2, 8, 3, 3, 1];
    counting_sort(&mut arr);
    println!("{:?}", arr);
}
