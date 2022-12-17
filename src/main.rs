use std::env;
use std::time::Instant;
mod sort;


fn main() {
    let args: Vec<String> = env::args().collect();

    let sizes = vec![10_000, 30_000, 90_000, 270_000, 810_000];

    for size in sizes.iter() {
        let mut vec: Vec<u16> = Vec::with_capacity(*size);
        for _ in 0..vec.capacity() {
            let num: u16 = rand::random();
            vec.push(num);
        }

        println!("array size: {}", size);

        // Bubble sort
        let mut sort_arr = vec.clone();

        let now = Instant::now();
        sort::bubble_sort(&mut sort_arr, *size);
        let elapsed = now.elapsed();

        println!("bubble_sort elapsed: {:.2?}", elapsed);
        if args.len() == 2 && args[1] == "print_result" {
            println!("result:  {:?}\n", sort_arr);
        }

        // Comb sort
        let mut sort_arr = vec.clone();

        let now = Instant::now();
        sort::comb_sort(&mut sort_arr, *size);
        let elapsed = now.elapsed();

        println!("comb_sort elapsed: {:.2?}", elapsed);
        if args.len() == 2 && args[1] == "print_result" {
            println!("result:  {:?}\n", sort_arr);
        }

        // Counting sort
        let mut sort_arr = vec.clone();

        let now = Instant::now();
        sort::counting_sort(&mut sort_arr, *size);
        let elapsed = now.elapsed();

        println!("counting_sort elapsed: {:.2?}", elapsed);
        if args.len() == 2 && args[1] == "print_result" {
            println!("result:  {:?}\n", sort_arr);
        }

        println!("\n");
    }

}

// ./target/release/lb21
// array size: 10000
// bubble_sort elapsed: 47.04ms
// comb_sort elapsed: 111.04ms
// counting_sort elapsed: 256.89µs
//
//
// array size: 30000
// bubble_sort elapsed: 875.76ms
// comb_sort elapsed: 1.27s
// counting_sort elapsed: 442.12µs
//
//
// array size: 90000
// bubble_sort elapsed: 9.59s
// comb_sort elapsed: 13.73s
// counting_sort elapsed: 693.47µs
//
//
// array size: 270000
// bubble_sort elapsed: 91.30s
// comb_sort elapsed: 118.90s
// counting_sort elapsed: 1.16ms
//
//
// array size: 810000
// bubble_sort elapsed: 838.53s
// comb_sort elapsed: 1059.64s
// counting_sort elapsed: 2.31ms
