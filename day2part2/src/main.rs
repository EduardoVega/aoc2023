use std::io::BufRead;

use anyhow::{Context, Result};

// 12 red cubes, 13 green cubes, and 14 blue cubes
// const  RED_CUBES: i32 = 12;
// const  GREEN_CUBES: i32 = 13;
// const  BLUE_CUBES: i32 = 14;

fn main() -> Result<()>{
    let path = "day2-input.txt";
    let f = std::fs::File::open(path).with_context(|| format!("Open input file {}", path))?;
    let mut total : i32 = 0;

    for line in std::io::BufReader::new(f).lines(){
        match line {
            Ok(line) => {
                let line_split: Vec<&str> = line.split(':').collect();

                let mut red_cubes = 0;
                let mut green_cubes = 0;
                let mut blue_cubes = 0;

                for set in line_split[1].split(';'){
                    for cubes in set.split(','){
                        let cubes_split: Vec<&str> = cubes.trim_start().split(' ').collect();
                        
                        let number = cubes_split[0].parse::<i32>().with_context(|| format!("parse number of cubes {}", cubes_split[0]))?;
                        let color = cubes_split[1];

                        match color {
                            "red" => {
                                if red_cubes < number {
                                    red_cubes = number;
                                }
                            }
                            "green" => {
                                if green_cubes < number {
                                    green_cubes = number;
                                }
                            }
                            "blue" => {
                                if blue_cubes < number {
                                    blue_cubes = number;
                                }
                            }
                            _ => {
                                ()
                            }
                        }
                    }
                }

                total += red_cubes * green_cubes * blue_cubes 
                    
            }
            Err(_) => {
                continue;
            }
        }
    }
    
    println!("Result: {}", total);
    Ok(())
}

