use anyhow::anyhow;
use readerr;

// card 1 -> 
// 	count_of_winning_numbers: 4
// 	count_of_copies: 1

// card 2 ->
// 	count_of_copies: 2
// 	count_of_winning_numbers: 2

// card 3 -> 
// 	count_of_copies: 4
// 	count_of_winning_numbers: 2

// card 4 ->
// 	count_of_copies: 8
// 	count_of_winning_numbers: 1

// card 5 ->  
// 	count_of_copies: 14
// 	count_of_winning_numbers: 0

// card 6 ->
// 	count_of_copies: 1
// 	count_of_winning_numbers: 0

// Total is sum of all copies of cards, 1 + 2 + 4...

#[derive(Debug)]
struct Scratchcard {
    card_name: String,
    count_of_winning_numbers: i32,
    count_of_copies: i32,
}

fn main() -> anyhow::Result<()>{
    let filename = "day4-input.txt";

    let mut scratchcard_information = Vec::<Scratchcard>::new();
    let mut total = 0;
    for (i, line) in readerr::read_lines(filename)?.iter().enumerate(){        
        let mut scratchcard = Scratchcard{
            count_of_winning_numbers: 0,
            count_of_copies: 1,
            card_name: format!("card {}", i+1),
        };
        
        // Get copies.
        if i > 0 {
            for b in (0..i).rev(){
                let s = match scratchcard_information.get(b){
                    Some(s) => s,
                    None => return Err(anyhow!("Error getting scratchcard using index")),
                };

                if i <= b + s.count_of_winning_numbers as usize{
                    scratchcard.count_of_copies += 1 * s.count_of_copies;
                }
            }
        }

        // Get winning cards.
        let (scratchcard_numbers, winning_numbers) = get_scratchcard_numbers_and_winning_numbers(line);

        for s in scratchcard_numbers {
            if winning_numbers.contains(&s){
                scratchcard.count_of_winning_numbers += 1;
            }
        }

        println!("{:?}", scratchcard);
        total += scratchcard.count_of_copies;
        scratchcard_information.push(scratchcard);
    }

    println!("Total: {}", total);

    Ok(())
}

fn get_scratchcard_numbers_and_winning_numbers(line: &String) -> (Vec::<i32>, Vec::<i32>){
    let mut scratchcard_numbers = Vec::<i32>::new();
    let mut winning_numbers = Vec::<i32>::new();
    let mut insert_into_winning_numbers = false;

    for word in line.split_whitespace() {
        match word.parse::<i32>(){
            Ok(num) => {
                if insert_into_winning_numbers {
                    winning_numbers.push(num);
                }else{
                    scratchcard_numbers.push(num);
                }
            },
            Err(_) => {
                if word == "|" {
                    insert_into_winning_numbers = true;
                
                }
            }
        }

    }

    (scratchcard_numbers, winning_numbers)
}