use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("nigerian_breweries.txt")?;

    // Define the drink categories
    let lager = ["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = ["Legend", "Turbo King", "Williams"];
    let non_alcoholic = ["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    file.write(b"Lager:\n")?;
    for drink in &lager {
        file.write(format!("{}\n", drink).as_bytes())?;
    }
    file.write(b"\n")?;

    file.write(b"Stout:\n")?;
    for drink in &stout {
        file.write(format!("{}\n", drink).as_bytes())?;
    }
    file.write(b"\n")?;

    file.write(b"Non-Alcoholic:\n")?;
    for drink in &non_alcoholic {
        file.write(format!("{}\n", drink).as_bytes())?;
    }

    println!("File successfully written!");

    Ok(())
}

