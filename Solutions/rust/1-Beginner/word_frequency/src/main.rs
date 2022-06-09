use std::io;
use std::collections::HashMap;

fn get_input(msg: &str) -> String {
    let mut input = String::new();

    loop {
        println!("{}", msg);

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        input = input.trim().to_string();

        if input.is_empty() {
            println!("Please enter a non-empty string");
        } else {
            break;
        }
    }

    return input;
}

fn analyse(text: String) -> Vec<(String, u64)> {
    let mut frequencies = HashMap::new();

    for word in text.split_whitespace() {
        let word = word.to_lowercase();

        let count = frequencies.entry(word).or_insert(0);
        *count += 1;
    }

    let mut frequencies = frequencies
        .iter()
        .map(|(word, count)| (word.to_string(), *count))
        .collect::<Vec<(String, u64)>>();

    frequencies.sort_by(|a, b| b.1.cmp(&a.1));

    return frequencies;
}

fn display(frequencies: &Vec<(String, u64)>) -> () {
    for (word, count) in frequencies {
        println!("{:.<20}{:.>20}", word, count);
    }
}

fn main() {
    let text = get_input("Enter text to be analysed").trim().to_string();

    let frequencies = analyse(text);

    display(&frequencies);
}
