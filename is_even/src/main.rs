use std::io;
use std::io::Write;
use std::fs::File;

fn main() {
    let max = greet();
    println!("Generating file");

    let mut print_string = String::from("def is_even(num):\n");

    // Add first line
    print_string.push_str("\tif num == 0: print(\"even!\")\n");
    for i in 1..=max {
        if i % 2 == 0 {
            print_string.push_str(&format!("\telif num == {}: print(\"even!\")\n", i));
        } else {
            print_string.push_str(&format!("\telif num == {}: print(\"odd!\")\n", i));
        }
    }
    // Close print_string
    print_string.push_str("\nis_even(int(input(\"Enter the number: \")))");

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
    println!("Hello! This will generate a solution to the FizzBuzz problem using the **best** coding practices.");
    println!("To run the the solution, make sure you have python installed and then run \"python3 solution.py\"");
    println!();
    println!("Now, lets get started. Enter the max number the solution should generate up to: ");

    // Loop while input is invalid
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input!");

        if let Ok(num) = input.trim().parse() {
            return num;
        } else {
            println!("Invalid input! Make sure it is a number!");
        }
    }
}
