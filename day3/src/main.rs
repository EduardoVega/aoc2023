use std::io::BufRead;

use anyhow::{Context, Result};

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Number {
    number: i32,
    head_index: usize,
    tail_index: usize,
}

fn main() -> Result<()>{
    let path = "sample.txt";
    let f = std::fs::File::open(path).with_context(|| format!("Open input file {}", path))?;

    let lines: Vec<String> = std::io::BufReader::new(f).lines().map(|l| l.unwrap()).collect();

    for i in 0..lines.len() {
        println!("{}", lines[i]);

        // Search for symbols in previous and next line
        // and store their index.

        // Search for numbers in a line and check if symbol is before or after
        // it.
        let mut number_string: String = "".to_string();

        let mut symbols_indexes: Vec<usize> = vec![];

        let mut numbers: Vec<Number> = vec![];

        let mut number_struct = Number{
            number: 0,
            head_index: 0,
            tail_index: 0,
        };

        let chars: Vec<char> = lines[i].chars().collect();
        for j in 0..chars.len(){
            // If number, we start concatinating until we find a symbol
            // or separator.
            // We also store head index.
            if chars[j].is_numeric() {
                if number_string == "" {
                    number_struct.head_index = j.to_owned();
                }

                number_string += &chars[j].to_string();

            // If separator or symbol.
            } else {
                // If not empty, we store number and reset it to start
                // over.
                if number_string.len() > 0 {
                    number_struct.number = number_string.parse::<i32>()?;
                    number_struct.tail_index = j.to_owned()-1;
                    numbers.push(number_struct);
                    
                    number_string = "".to_string();
                }

                // If symbol.
                if chars[j] != '.' {
                    symbols_indexes.push(j);
                }
            }
        }

        println!("{:?} -- {:?}", numbers, symbols_indexes);
    }
    
    Ok(())
}