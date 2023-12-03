use advent_of_code_2023::read_file::read_file;




fn is_game_valid(line: String) -> Vec<u32> {
    let binding = line.split(&[':', ' ', ',', ';'][..]).collect::<Vec<&str>>();
    let mut it = binding.into_iter();
    //red, green, blue
    let mut res = vec![0, 0, 0];
    let mut _idx = 0;

    
    let mut parse_num;
    let mut num;
    let mut color;
    
    while let Some(word) = it.next(){
        match word {
            "Game" => {it.next(); it.next();},
            x => {
                parse_num = x.parse::<u32>();
                if !parse_num.is_err() {
                    color = it.next().unwrap();
                    num = parse_num.unwrap();
                    match color {
                        "red" => _idx = 0,
                        "green" => _idx = 1,
                        "blue" => _idx = 2,
                        _ => panic!("Wrong color")
                    }
                    if res[_idx] < num {res[_idx] = num}
                }
            }
        }
    }
    res
}

fn sum_powers(powers: Vec<u32>) -> u32{
    let mut sum = 1;
    for p in powers {
        sum *= p;
    }
    sum
}





fn main() {
    let lines = read_file("input/day2.txt");

    let mut sum = 0;
    let mut i = 1;
    for line in lines {
        sum += sum_powers(is_game_valid(line));
        println!("Game {}: {}", i, sum);
        i +=1;
    }

    println!("{}",sum)

}