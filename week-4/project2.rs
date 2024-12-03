use std::io;

fn main() {
    let mut input = String::new();
    let experience: String;
    let age: i32;

    println!("Is the employee experienced? (yes/no):");
    io::stdin().read_line(&mut input).unwrap();
    experience = input.trim().to_string();
    input.clear();

    println!("Enter the employee's age:");
    io::stdin().read_line(&mut input).unwrap();
    age = input.trim().parse().unwrap();

    let incentive = if experience == "yes" {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else {
            1_300_000
        }
    } else {
        100_000
    };

    println!("The employee's annual incentive is: N{}", incentive);
}