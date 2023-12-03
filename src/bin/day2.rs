use std::u32;

use advent_of_code_2023::read_file::read_file;




fn is_game_valid(line: String, game_config: &Vec<(u32, &str)>) -> u32 {
    let binding = line.split(&[':', ' ', ',', ';'][..]).collect::<Vec<&str>>();
    let mut it = binding.into_iter();
    let original_iter = it.clone();

    
    let mut id = 0;
    let mut parse_num;
    let mut num;
    let mut color;
    for conf in game_config {
        while let Some(word) = it.next(){
            match word {
                "Game" => id = it.next().unwrap().parse::<u32>().unwrap(),
                x => {
                    parse_num = x.parse::<u32>();
                    if !parse_num.is_err() {
                        color = it.next().unwrap();
                        num = parse_num.unwrap();
                        if conf.1 == color && conf.0 < num {
                            return 0
                        }
                    }
                }
            }
        }
        it = original_iter.clone().into_iter();
    }
    id
}





fn main() {
    let lines = read_file("input/day2.txt");
    let game_config: Vec<(u32, &str)> = vec![(12, "red"), (13, "green"), (14, "blue")];

    let mut sum = 0;
    let mut i = 1;
    for line in lines {
        sum += is_game_valid(line, &game_config);
        println!("Game {}: {}", i, sum);
        i +=1;
    }

    println!("{}",sum)

}