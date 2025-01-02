fn main() {
    // Create an empty vector "City"
    let mut city: Vec<String> = Vec::new();

    // Print City Vector
    println!("The city vector has element {}", city.len());

    // Push new elements into it
    let mut input1 = String::new();
    println!("How many cities do you want to enter?");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num: i32 = input1.trim().parse().expect("Invalid input");

    for count in 0..city_num {
        let mut input2 = String::new();
        println!("Enter City {}:", count + 1);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_city: String = input2.trim().parse().expect("Invalid input");
        city.push(new_city);
    }

    print!("Your preferred cities are:\n");

    let mut count = city_num - 1;
    // Loop to iterate elements in the vector
    for i in city 
    {
        // Iterating through i on the vector
        println!("{} {}", count, i);
        count += 1;
    }
}
