use std::io::BufRead;

use anyhow::{Context, Result};

// 12 red cubes, 13 green cubes, and 14 blue cubes
const  RED_CUBES: i32 = 12;
const  GREEN_CUBES: i32 = 13;
const  BLUE_CUBES: i32 = 14;

fn main() -> Result<()>{
    let path = "day2-input.txt";
    let f = std::fs::File::open(path).with_context(|| format!("Open input file {}", path))?;
    let mut total : i32 = 0;

    for line in std::io::BufReader::new(f).lines(){
        match line {
            Ok(line) => {
                let line_split: Vec<&str> = line.split(':').collect();

                let game: Vec<&str> = line_split[0].split(' ').collect();
                let game_id = game[1];

                let mut game_ok = true;

                for set in line_split[1].split(';'){
                    let mut red_cubes = 0;
                    let mut green_cubes = 0;
                    let mut blue_cubes = 0;

                    for cubes in set.split(','){
                        let cubes_split: Vec<&str> = cubes.trim_start().split(' ').collect();
                        
                        let number = cubes_split[0];
                        let color = cubes_split[1];

                        match color {
                            "red" => {
                                red_cubes = number.parse::<i32>().with_context(|| format!("parse number of cubes {}", number))?;
                            }
                            "green" => {
                                green_cubes = number.parse::<i32>().with_context(|| format!("parse number of cubes {}", number))?;
                            }
                            "blue" => {
                                blue_cubes = number.parse::<i32>().with_context(|| format!("parse number of cubes {}", number))?;
                            }
                            _ => {
                                ()
                            }
                        }
                    }

                    if red_cubes > RED_CUBES || green_cubes > GREEN_CUBES || blue_cubes > BLUE_CUBES{
                        game_ok = false;
                        break;
                    }
                }

                if game_ok{
                    total += game_id.parse::<i32>().with_context(|| format!("parse game id {}", game_id))?;
                }
                    
            }
            Err(_) => {
                continue;
            }
        }
    }
    
    println!("Result: {}", total);
    Ok(())
}

