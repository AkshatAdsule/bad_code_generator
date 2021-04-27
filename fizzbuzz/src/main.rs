use std::io;
use std::io::Write;
use std::fs::File;

fn main() {
    let max = greet();
    println!("Generating file");

    let mut print_string = String::from("print(\"\"\"\n");
    for i in 0..=max {
        if i % 15 == 0 {
            print_string.push_str("FizzBuzz\n");
            continue;
        }

        if i % 3 == 0 {
            print_string.push_str("Fizz\n");
            continue;
        }

        if i % 5 == 0 {
            print_string.push_str("Buzz\n");
            continue;
        }

        print_string.push_str(&i.to_string());
        print_string.push('\n');
    }
    // Close print_string
    print_string.push_str("\"\"\")");

    // Write to file
    let mut file = match File::create("solution.py") {
        Ok(file) => file,
        Err(_) => panic!("Could not create file!")
    };

    match file.write_all(print_string.as_bytes()) {
        Ok(_) => println!("Finished! run \"python3 solution.py\""),
        Err(_) => panic!("Could not write to file!")
    };

}

fn greet() -> u32 {
    println!("Hello! This will generate a solution to the FizzBuzz problem using the **best** coding practices. (guaranteed O(1) time complexity!)");
    println!("To run the the solution, make sure you have python installed and then run \"python3 solution.py\"");
    println!();
    println!("Now, lets get started. Enter the max number the solution should fizzbuzz up to: ");

    // Loop while input is invalid
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input!");

        if let Ok(num) = input.trim().parse() {
            return num;
        }
    }
}
