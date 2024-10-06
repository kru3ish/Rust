use std::io; // imports the standard library's input/output module so we can take user input

fn main() {
    // user for first number
    println!("Enter the first number ");
    let mut first_number = String::new(); // creates a mutable string to store the users input
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read input"); // reads a line from the standard input and stores it in the variable
    let first_number: f64 = first_number
        .trim()
        .parse()
        .expect("Please enter a valid number"); // .trim to remove white space .parse to convert the string into a float .expect so that we ensure that the user enters a valid number
                                                // user for second number
    println!("Enter the second number ");
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read input");
    let second_number: f64 = second_number
        .trim()
        .parse()
        .expect("Please enter a valid number");

    // asking the user for operation
    println!("Enter the operation (+,-,*,/):");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read input");

    // below we are using the match statement to check what operation has the user input
    let result = match operation.trim() {
        //.trim is necessary as io::stdin() adds a \n at the end
        "+" => first_number + second_number,
        "-" => first_number - second_number,
        "*" => first_number * second_number,
        "/" => first_number / second_number,
        _ => {
            //wildcard
            println!("Invalid Operation");
            return;
        }
    };

    println!("The result is: {}", result);
}
