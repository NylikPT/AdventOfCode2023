use advent_of_code_2023::read_file::read_file;


fn search_digits (line: String) -> ((i32, i32), (i32, i32)){
    //((first_digit, pos), (last_digit, pos))
    let mut digits = ((-1, i32::MAX),(-1, i32::MIN));
    let mut tmp_first;
    let mut first_pos;
    let mut tmp_last;
    let mut last_pos;
    
    let aux = [("one", 1), ("1", 1), ("two", 2), ("2", 2), 
                                  ("three", 3), ("3", 3), ("four", 4), ("4", 4), 
                                  ("five", 5), ("5", 5), ("six", 6), ("6", 6), 
                                  ("seven", 7), ("7", 7), ("eight", 8), ("8", 8), 
                                  ("nine", 9), ("9", 9)];

    for (digit, value) in aux.iter() {                 
        let one: Vec<_> = line.match_indices(digit).collect();
        if one.len() > 0 {
            tmp_first = one.first().unwrap();
            first_pos = tmp_first.0 as i32;
            if digits.0.1 > first_pos{
                digits.0.0 = value.clone();
                digits.0.1 = first_pos;
            }
            
            if one.len() == 1 {
                if digits.1.1 < first_pos {
                    digits.1.0 = value.clone();
                    digits.1.1 = first_pos;
                }
            } 
            else {
                tmp_last = one.last().unwrap();
                last_pos = tmp_last.0 as i32;
                if digits.1.1 < last_pos {
                    digits.1.0 = value.clone();
                    digits.1.1 = last_pos;
                }
            }

        }
    }

    digits
}

fn compute_cal_value(line: String) -> u32{
    let ((first_digit, _), (last_digit, _)) = search_digits(line);
    
    (first_digit*10 + last_digit) as u32
}






fn main() {
    let lines = read_file("input/day1.txt");

    let mut sum = 0;

    let mut i = 1;
    for line in lines {
        sum += compute_cal_value(line);
        println!("Line {}: {}", i, sum);
        i += 1;
    }

    print!("{}", sum);

}
