fn main() {
   
    let principal: f64 = 520_000_000.0; 
    let rate: f64 = 0.10; 
    let time: f64 = 5.0; 
    let n: f64 = 1.0; 

    let amount = principal * (1.0 + rate / n).powf(n * time);
    let compound_interest = amount - principal;

  
    println!("Total amount after 5 years: ₦{:.2}", amount);
    println!("Compound interest accrued: ₦{:.2}", compound_interest);
}
