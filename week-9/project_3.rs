use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // 1. Define the datasets as vectors.
    let names = vec![
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbona",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etieye",
    ];
    let zones = vec!["South West", "North East", "South South", "South West", "South East"];
    let ministries = vec![
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power & Steel",
        "Petroleum",
    ];

    // 2. Create a file named "merged_ministers.txt".
    let mut file = File::create("merged_ministers.txt")?;

    // 3. Merge the datasets and write to the file.
    file.write_all(b"S/N\tNAME OF COMMISSIONER\tGEOPOLITICAL ZONE\tMINISTRY\n")?;
    for i in 0..names.len() {
        let line = format!(
            "{}\t{}\t{}\t{}\n",
            i + 1,
            names[i],
            zones[i],
            ministries[i]
        );
        file.write_all(line.as_bytes())?;
    }

    Ok(())
}