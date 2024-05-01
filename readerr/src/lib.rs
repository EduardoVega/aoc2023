use std::{default, fmt, fs::{read, File}, io::{self, Read}, string};
use anyhow::{self, Context};

pub fn read_lines(filename: &str) -> anyhow::Result<Vec<String>>{
    let mut f = File::open(filename)
        .with_context(|| format!("open file {}", filename))?;

    let mut lines: Vec<String> = Vec::new();
    let mut buf = vec![0; 10];
    let mut line = String::new();
    loop {
        let b = f.read(&mut buf[..])?;

        if b == 0 {
            lines.push(line);
            break;
        }

        //println!("{}", std::str::from_utf8(&buf[..b])?);
        for c in std::str::from_utf8(&buf[..b])?.chars(){
            match c {
                '\n' => {
                    //println!(" ");
                    lines.push(line);
                    line = String::new();
                },
                _ => {
                    //println!("{}", c);
                    line = line.to_owned() + &c.to_string();
                }
            }
        }
    }

    Ok(lines)
}
