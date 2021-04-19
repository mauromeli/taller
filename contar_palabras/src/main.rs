use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    // --snip--
    let filename = "src/file.txt"; 
    let mut word_count : HashMap<String, i8> = HashMap::new();

    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        for word in line.split(" ") {
            match word_count.get(word) {
                Some(count) => word_count.insert((&word).to_string(), count + 1),
                None => word_count.insert((&word).to_string(), 1)
            };
        }
    }


    // Step 2: get Vector from HashMap.
    let mut sorted: Vec<_> = word_count.iter().collect();

    // Step 3: sort Vector by key from HashMap.
    // ... This sorts by HashMap keys.
    //     Each tuple is sorted by its first item.
    sorted.sort_by_key(|a| -a.1);

    // Step 4: loop over sorted vector.
    for (key, value) in sorted.iter() {
        println!("{} -> {}", key, value);
    }
}