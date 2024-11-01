fn main() {
    
    let initial_value: f64 = 510_000.0;

    
    let depreciation_rate: f64 = 0.05;

    
    let years: i32 = 3;

   
    let final_value = initial_value * (1.0 - depreciation_rate).powi(years);

    
    println!("The value of the TV after {} years is: N{:.2}", years, final_value);
}
