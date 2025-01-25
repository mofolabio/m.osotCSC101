#[derive(Debug)] 
struct Laptop {
    brand: String,
    cost: i32,
}

impl Laptop {
    fn calculate_cost(&self, quantity: i32) -> i32 {
        self.cost * quantity
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        cost: 650000,
    };
    let ibm = Laptop {
        brand: String::from("IBM"),
        cost: 755000,
    };
    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        cost: 550000,
    };
    let dell = Laptop {
        brand: String::from("Dell"),
        cost: 850000,
    };


    println!(
        "The total cost of 3 {} laptops: {}",
        hp.brand,
        hp.calculate_cost(3)
    );
    println!(
        "The total cost of 3 {} laptops: {}",
        ibm.brand,
        ibm.calculate_cost(3)
    );
    println!(
        "The total cost of 3 {} laptops: {}",
        toshiba.brand,
        toshiba.calculate_cost(3)
    );
    println!(
        "The total cost of 3 {} laptops: {}",
        dell.brand,
        dell.calculate_cost(3)
    );

   
    let total_cost = hp.calculate_cost(3)
        + ibm.calculate_cost(3)
        + toshiba.calculate_cost(3)
        + dell.calculate_cost(3);

    println!("Total cost for purchasing 3 of each brand: {}", total_cost);
}
