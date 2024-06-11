// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// I AM NOT DONE

fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    number = "3"; // Assigning a string representation of the number 3
    let parsed_number: i32 = number.parse().expect("Failed to parse number"); // Convert string to integer
    let result = parsed_number + 2; // Add 2 to the parsed number
    println!("Number plus two is : {}", result);
}