use advent_of_code_2023::read_file::read_file;

fn compute_cal_value(line: String) -> u32{
    let iter = line.chars();
    
    let mut first_digit = None;
    let mut last_digit = None;

    for char in iter {
        let c = char.to_digit(10);
        match c {
            Some(_) => if first_digit == None {first_digit = c} else {last_digit = c},
            None => continue

        }
    }
    
    first_digit.unwrap()*10 + if last_digit == None {first_digit.unwrap()} else {last_digit.unwrap()}
}






fn main() {
    let lines = read_file("input/day1.txt");

    let mut sum = 0;

    for line in lines {
        sum += compute_cal_value(line);
    }

    print!("{}", sum);

}
