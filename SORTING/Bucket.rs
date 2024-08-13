// 9. Bucket Sort
fn bucket_sort(arr: &mut [f64]) {
    if arr.is_empty() {
        return;
    }

    let num_buckets = arr.len();
    let mut buckets: Vec<Vec<f64>> = vec![Vec::new(); num_buckets];

    let max_value = arr.iter().cloned().fold(f64::MIN, f64::max);
    let min_value = arr.iter().cloned().fold(f64::MAX, f64::min);

    for &value in arr.iter() {
        let bucket_index = ((value - min_value) / (max_value - min_value) * (num_buckets as f64)) as usize;
        buckets[bucket_index.min(num_buckets - 1)].push(value);
    }

    for bucket in &mut buckets {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    arr.clear();
    for bucket in buckets {
        arr.extend(bucket);
    }
}

fn main() {
    let mut arr = [0.897, 0.565, 0.656, 0.1234, 0.665, 0.3434];
    bucket_sort(&mut arr);
    println!("{:?}", arr);
}
