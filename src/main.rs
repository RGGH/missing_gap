/// Based on CodeWars - Find the missing term in an Arithmetic Progression

use std::collections::HashMap;

fn find_missing(seq: &[i32]) -> (i32, usize) {
    let mut hm: HashMap<i32, i32> = HashMap::new();

    // Calculate gaps and count their occurrences
    for i in 1..seq.len() {
        let previous = seq[i - 1];
        let gap = seq[i] - previous;
        *hm.entry(gap).or_insert(0) += 1;
    }
    println!("Number : Frequency {:?}\n", hm);

    // Closure to identify and return the large gap
    let res = || {
        for (&k, &v) in &hm {
            if v == 1 {
                return k; // Return the gap value if it appears only once
            }
        }
        0 // Default return value if no single large gap is found
    };

    let big_gap = res(); // Call the closure to get the 'big gap'

    // Find the position where the big gap occurs and calculate the missing number
    for i in 1..seq.len() {
        if seq[i] - seq[i - 1] == big_gap {
            let missing_number = seq[i - 1] + big_gap / 2;
            let location = i; // The location where the missing number would fit
            println!("Missing number: {}, Location: {}", missing_number, location);
            return (missing_number, location); // Return the missing number and location
        }
    }
    (-1, 0) // Return a default tuple if no missing number is found
            
}

fn main() {
    let missing_gap = find_missing(&[2, 3, 4, 5, 7, 8, 9]);
    println!(
        "Size of the 'missing' gap: = {}, location = {}",
        missing_gap.0, missing_gap.1
    );
}
