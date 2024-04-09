use std::io::BufRead;

use anyhow::{Result, Context};

fn main() -> Result<()>{
    let path = "day1-input.txt";
    let f = std::fs::File::open(path).with_context(|| format!("Open input file {}", path))?;
    let mut total : u32 = 0;

    for line in  std::io::BufReader::new(f).lines(){
        let mut calibration_value = String::new();

        match line {
            Ok(line) => {
                for c in line.chars(){
                    if c.is_digit(10){
                        calibration_value = c.to_string();
                        break;
                    }
                }

                for c in line.chars().rev(){
                    if c.is_digit(10){
                        calibration_value = calibration_value + &c.to_string();
                        break;
                    }
                }
            }
            Err(_) => {
                continue;
            }
        }

        total += calibration_value.parse::<u32>().with_context(|| format!("Parse calibration value {}", calibration_value))?;
    }
    
    println!("Result: {}", total);
    Ok(())
}
