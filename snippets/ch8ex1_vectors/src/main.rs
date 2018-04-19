extern crate rand;

use std::collections::HashMap;use rand::Rng;

fn main() {

/*
    // create a 100 elements vector and fills it
    // with random values
    let mut v: Vec<i32> = Vec::new();
    for _ in 0..101 {
        v.push(rand::thread_rng().gen_range(1, 101));
    }

    // Computes mean value
    let mut mean = 0.0;
    for element in v.iter() {
        mean += (*element as f64) / 101.0;
    }
    println!("Mean value is: {}", mean);

    // Prints median
    v.sort();    // sorts values
    println!("Median value is: {}", v[50]);

    // Computes mode
    let mut freqs = HashMap::new();
    for index in 0..101 {
        let count = freqs.entry(v[index]).or_insert(0);
        *count += 1;
    }

    let mut max_freq = 0;
    for (_, val) in &freqs {
        if val > &max_freq {
            max_freq = *val;
        }
    }
    println!("MaxFreq = {}", max_freq);

    for (key, val) in &freqs {
        if val == &max_freq {
            println!("Moda value = {}", key);
        }
    }
*/
    // ----------------------------------------------------
    // Optimized version of the previous code
    println!("-------------------------------------");
    let mut v: Vec<i32> = Vec::new();
    let mut mean = 0.0;
    let mut freqs = HashMap::new();
    let mut max_freq = 0;
    for index in 0..101 {
        v.push(rand::thread_rng().gen_range(1, 101));
        mean += (v[index] as f64) / 101.0;
        let count = freqs.entry(v[index]).or_insert(0);
        *count += 1;
        if *count > max_freq {
            max_freq = *count;
        }
    }

    // Prints mean value
    println!("Mean value is: {}", mean);

    // Prints median
    v.sort();    // sorts values
    println!("Median value is: {}", v[50]);

    // Computes and prints mode value(s)
    println!("Max frequency for moda: {}", max_freq);
    for (key, val) in &freqs {
        if val == &max_freq {
            println!(" - Moda value = {}", key);
        }
    }




}
