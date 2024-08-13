fn cycle_sort(arr: &mut [i32]) {
    let mut writes = 0;

    for cycle_start in 0..arr.len() {
        let item = arr[cycle_start];

        let mut pos = cycle_start;
        for i in (cycle_start + 1)..arr.len() {
            if arr[i] < item {
                pos += 1;
            }
        }

        if pos == cycle_start {
            continue;
        }

        while item == arr[pos] {
            pos += 1;
        }

        arr.swap(pos, cycle_start);
        writes += 1;

        let mut item = arr[pos];
        while pos != cycle_start {
            pos = cycle_start;
            for i in (cycle_start + 1)..arr.len() {
                if arr[i] < item {
                    pos += 1;
                }
            }

            while item == arr[pos] {
                pos += 1;
            }

            arr.swap(pos, cycle_start);
            writes += 1;
            item = arr[pos];
        }
    }
}

fn main() {
    let mut arr = [5, 2, 9, 1, 5, 6];
    cycle_sort(&mut arr);
    println!("{:?}", arr);
}
