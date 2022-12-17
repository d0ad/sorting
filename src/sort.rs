pub fn bubble_sort<T: Ord>(arr: &mut [T], size: usize) {
    for i in 0..size {
        for j in 0..size - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

pub fn comb_sort<T: Ord>(arr: &mut [T], size: usize) {
    let mut gap = size as f64;
    let shrink = 2;
    let mut sorted = false;

    while !sorted {
        gap = gap as f64 / shrink as f64;
        gap = gap.floor();

        if gap <= 1.0 {
            gap = 1.0;
            sorted = true;
        }

        let mut i = 0.0;
        while i + gap < size as f64 {
            if arr[i as usize] > arr[i as usize + gap as usize] {
                arr.swap(i as usize, i as usize + gap as usize);
                sorted = false;
            }
            i += 1.0;
        }
    }
}

pub fn counting_sort(arr: &mut [u16], size: usize) {
    let mut max_val: u32 = arr[0].into();
    for val in arr.iter().take(size).skip(1) {
        if *val as u32 > max_val {
            max_val = *val as u32;
        }
    }

    let mut occurences: Vec<usize> = vec![0; (max_val + 1) as usize];

    for val in arr.iter().take(size) {
        occurences[*val as usize] += 1;
    }

    let mut i = 0;
    for (data, &number) in occurences.iter().enumerate() {
        for _ in 0..number {
            arr[i] = data as u16;
            i += 1;
        }
    }
}
