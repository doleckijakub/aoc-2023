use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err("Missing filename".into());
    }

    let filename = &args[1];
    let file = File::open(filename).map_err(|e| {
        format!("Error opening file: {}: {}", filename, e)
    })?;

    let reader = io::BufReader::new(file);

    let mut sum: u32 = 0;

    for line in reader.lines().map(|l| l.unwrap()) {
        if let Some(index) = line.find(char::is_numeric) {
            let digit = line.chars().nth(index).unwrap();
            sum += 10 * (digit as u32 - '0' as u32);
        } else {
            eprintln!("No digits found in \"{}\"!", line)
        }
        
        if let Some(index) = line.rfind(char::is_numeric) {
            let digit = line.chars().nth(index).unwrap();
            sum += digit as u32 - '0' as u32;
        } else {
            eprintln!("No digits found in \"{}\"!", line)
        }
    }

    println!("{}", sum);

    Ok(())
}
