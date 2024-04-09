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
                let mut letters = String::new();
                let mut digits_in_line: Vec<String> = Vec::new();

                for c in line.chars(){
                    if c.is_digit(10){
                        digits_in_line.push(c.to_string());
                        continue;
                    }

                    // If letter, we concatenate and search for a string
                    // that matches a digit in letters.
                    letters = letters + &c.to_string();

                    let res = match_digit(&letters);
                    match res {
                        Some(digit) =>{
                            digits_in_line.push(digit.to_string());
                            letters = c.to_string();
                        }
                        None => ()
                    }
                }

                calibration_value = digits_in_line[0].as_str().to_owned() + &digits_in_line[digits_in_line.len()-1];
                //println!("{} => {}", line ,calibration_value);
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

fn match_digit(letters: &str) -> Option<&'static str> {
    if letters.contains("one"){
        return Some("1");
    }

    if letters.contains("two"){
        return Some("2");
    }

    if letters.contains("three"){
        return Some("3");
    }

    if letters.contains("four"){
        return Some("4");
    }

    if letters.contains("five"){
        return Some("5");
    }

    if letters.contains("six"){
        return Some("6");
    }

    if letters.contains("seven"){
        return Some("7");
    }

    if letters.contains("eight"){
        return Some("8");
    }

    if letters.contains("nine"){
        return Some("9");
    }

    None
}