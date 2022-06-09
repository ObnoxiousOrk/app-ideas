use std::io;

fn bin_to_dec(bin: &str) -> i32 {
    let mut dec = 0;
    let mut power = 0;

    for c in bin.chars().rev() {
        dec += (c.to_digit(10).unwrap() as i32) * 2i32.pow(power);
        power += 1;
    }
    return dec
}

fn get_input() -> String {
    
    loop {
        let mut input = String::new();

        println!("Enter a binary number:");

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        let input = input.trim().to_string();

        if input.chars().all(|c| c.eq(&'0') || c.eq(&'1')) {
            return input;
        } else {
            println!("Please enter a binary number (should contain only 0 and 1)");
        }
    }
}

fn main() {
    let bin = get_input();

    println!("{} in decimal is {}", bin, bin_to_dec(&bin));
}
