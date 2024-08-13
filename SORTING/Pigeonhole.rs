fn pigeonhole_sort(arr: &mut [i32]) {
    if arr.is_empty() {
        return;
    }

    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    let range = (max - min + 1) as usize;

    let mut holes = vec![vec![]; range];

    for &num in arr.iter() {
        holes[(num - min) as usize].push(num);
    }

    let mut index = 0;
    for hole in holes.iter() {
        for &num in hole.iter() {
            arr[index] = num;
            index += 1;
        }
    }
}

fn main() {
    let mut arr = [8, 3, 2, 7, 4, 6, 1];
    pigeonhole_sort(&mut arr);
    println!("{:?}", arr);
}
