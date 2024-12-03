use std::io;

fn main() {
    println!("Welcome to our restaurant!");
    println!("Menu:");
    println!("P = Poundo Yam/Edinkaiko Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    loop {
        println!("Enter the food type (P, F, A, E, W) or Q to quit:");

        let mut food_type = String::new();
        io::stdin()
            .read_line(&mut food_type)
            .expect("Failed to read line");

        let food_type = food_type.trim().to_uppercase();

        if food_type == "Q" {
            break;
        }

        let price = match food_type.as_str() {
            "P" => 3200,
            "F" => 3000,
            "A" => 2500,
            "E" => 2000,
            "W" => 2500,
            _ => {
                println!("Invalid food type.");
                continue;
            }
        };

        println!("Enter the quantity:");

        let mut quantity = String::new();
        io::stdin()
            .read_line(&mut quantity)
            .expect("Failed to read line");

        let quantity: u32 = match quantity.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid quantity.");
                continue;
            }
        };

        let total_cost = price * quantity;

        let discount = if total_cost > 10000 {
            total_cost * 5 / 100
        } else {
            0
        };

        println!("Total cost: N{}", total_cost);
        println!("Discount: N{}", discount);
        println!("Final cost: N{}", total_cost - discount);
    }

    println!("Thank you for your order!");
}