use std::io;

fn main() {
    let mut input = String::new();
    let mut a: f64;
    let mut b: f64;
    let mut c: f64;

    println!("Enter the value of a:");
    io::stdin().read_line(&mut input).unwrap();
    a = input.trim().parse().unwrap();
    input.clear();

    println!("Enter the value of b:");
    io::stdin().read_line(&mut input).unwrap();
    b = input.trim().parse().unwrap();
    input.clear();

    println!("Enter the value of c:");
    io::stdin().read_line(&mut input).unwrap();
    c = input.trim().parse().unwrap();

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b + discriminant.sqrt()) / (2.0 * a);
        println!("The roots are: {} and {}", root1, root2);
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("The root is: {}", root);
    } else {
        println!("The equation has no real roots.");
    }
}