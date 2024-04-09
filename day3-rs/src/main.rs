use std::io::BufRead;

use anyhow::{Context, Result};

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Number {
    number: i32,
    head_index: usize,
    tail_index: usize,
    row: usize,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Symbol {
    index: usize,
    row: usize,
}

fn main() -> Result<()>{
    let path = "day3-input.txt";
    let f = std::fs::File::open(path).with_context(|| format!("Open input file {}", path))?;

    let lines: Vec<String> = std::io::BufReader::new(f).lines().map(|l| l.unwrap()).collect();

    let mut result: i32 = 0;

    let mut numbers: Vec<Number> = vec![];

    let mut symbols: Vec<Symbol> = vec![];

    // First we identify numbers and symbols and their location in the matrix.
    for i in 0..lines.len() {
        let mut number_string: String = "".to_string();

        let mut number_struct = Number{
            number: 0,
            head_index: 0,
            tail_index: 0,
            row: 0,
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

                // If number is at the end of the line.
                if j == chars.len()-1 {
                    number_struct.row = i;
                    number_struct.number = number_string.parse::<i32>()?;
                    number_struct.tail_index = j.to_owned();
                    numbers.push(number_struct);
                }

            // If separator or symbol.
            } else {
                // If not empty, we store number and reset it to start
                // over.
                if number_string.len() > 0 {
                    number_struct.row = i;
                    number_struct.number = number_string.parse::<i32>()?;
                    number_struct.tail_index = j.to_owned()-1;
                    numbers.push(number_struct);
                    
                    number_string = "".to_string();
                }

                // If symbol.
                if chars[j] != '.' {
                    symbols.push(Symbol{
                        index: j,
                        row: i,
                    });
                }
            }
        }
    }

    for number in numbers.iter() {
        let mut adjacent: bool = false;

        for symbol in symbols.iter(){
            let min_row:usize;
            if symbol.row == 0 {
                min_row = 0;
            }else{
                min_row = symbol.row-1;
            }

            if number.row == min_row || number.row == symbol.row || number.row == symbol.row+1 {
                let mut head: usize = number.head_index; 
                if number.head_index != 0 {
                    head = head-1;
                }
                
                if symbol.index >= head && symbol.index <= number.tail_index+1 {
                    result += number.number;
                    
                    adjacent = true;
                    
                    println!("{} yes", number.number);
                    
                    // If number is adjacent to a symbol, we break the loop and move to the next number.
                    break;
                }
            }
        }

        if !adjacent{
            println!("{} no", number.number);
        }
    }

    println!("{}", result);
    
    
    Ok(())
}