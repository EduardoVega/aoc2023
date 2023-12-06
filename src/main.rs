use std::io::{BufReader, BufRead};

use std::fs::File;

use std::string::String;

use anyhow::{Result, Ok};

fn main() -> Result<()>{
    day1()?;

    day1p2()?;

    Ok(())
}

fn day1() -> Result<()> {
    let input = "day1-input.txt";
    let mut total: u32 = 0;

    for line in BufReader::new(File::open(input)?).lines(){
        let mut first: String = String::from("");
        let mut last: String = String::from("");
        
        let l = line?;

        for c in l.chars(){
            match c.to_digit(10) {
                Some(d) => {
                    first = d.to_string();
                    break
                },
                None => continue
            }
        }

        for c in l.chars().rev(){
            match c.to_digit(10) {
                Some(d) => {
                    last = d.to_string();
                    break
                },
                None => continue
            }
        }

        total += (first.to_owned()+&last).parse::<u32>()?;
    }

    println!("Day 1 part 1: {}", total);

    Ok(())
}

fn day1p2() -> Result<()>{
    let input = "day1-input.txt";

    let mut total: u32 = 0;

    for line in BufReader::new(File::open(input)?).lines(){
        
        let mut letters: String = String::from("");

        let mut digits: Vec<u32> = Vec::new();
        
        let l = line?;

        for c in l.chars(){
            match c.to_digit(10) {
                Some(d) => {
                    digits.push(d);
                }
                None => {
                    letters = letters.to_owned() + &c.to_string();
                    let digit = find_digit(&letters);

                    match digit {
                        Some(d) => {
                            digits.push(d);
                            match letters.to_owned().chars().into_iter().last(){
                                Some(c) => {
                                    letters = String::from(c);
                                }
                                None => {
                                    letters = String::from("");
                                }
                            }
                        }
                        None => (),
                    }
                }
            }
        }
    
        println!("{:#?} {:?} {}", l, digits, digits[0]*10 + digits[digits.len()-1]);
        total += digits[0]*10 + digits[digits.len()-1];
    }

    println!("Day 1 part 2: {}", total);
    
    Ok(())
}

fn find_digit(letters: &str) -> Option<u32>{
    for digit in vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]{
        if letters.contains(digit){
            match digit {
                "one" => return Some(1),
                "two" => return Some(2),
                "three" => return Some(3),
                "four" => return Some(4),
                "five" => return Some(5),
                "six" => return Some(6),
                "seven" => return Some(7),
                "eight" => return Some(8),
                "nine" => return Some(9),
                _ => return None::<u32>
            }
        }
    }

    return None::<u32>
}