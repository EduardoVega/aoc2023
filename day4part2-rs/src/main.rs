use readerr;

fn main() -> anyhow::Result<()>{
    let filename = "day4-input.txt";

    for line in readerr::read_lines(filename)?{
        println!("{}", line);
    }

    Ok(())
}