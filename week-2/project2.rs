fn main() {
   
    let sales: [f64; 5] = [
        450_000.00,
        1_500_000.00,
        750_000.00,
        2_850_000.00,
        250_000.00,
    ];

  
    let sum: f64 = sales.iter().sum();

    
    let average: f64 = sum / sales.len() as f64;

    println!("Total Sales: ${:.2}", sum);
    println!("Average Sales: ${:.2}", average);
}fn main() {
    // Initial value of the TV
    let initial_value: f64 = 510_000.0;

    // Annual depreciation rate
    let depreciation_rate: f64 = 0.05;

    // Number of years
    let years: i32 = 3;

    // Calculate the value of the TV after 3 years
    let final_value = initial_value * (1.0 - depreciation_rate).powi(years);

    // Print the results
    println!("The value of the TV after {} years is: N{:.2}", years, final_value);
}
