use std::fs::read_to_string;
fn main() {
    let lines = read_lines("day1_input");
    for line in lines{
        if line.len() <4 {
            println!("{}", line)
        }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }
    result
}

fn get_first_and_last_digit(input: String) -> (char, char) {
    let mut first_digit = 'a';
    let mut last_digit = 'a';
    for character in input.chars(){
        if character.is_digit(10) {
            if first_digit == 'a' {
                first_digit = character;
                last_digit = character;
            }
            else { last_digit = character }
        }
    }
    (first_digit, last_digit)
}

fn get_number_from_first_and_last(input: String) -> i32 {
    let num_chars = get_first_and_last_digit(input);
    let mut nums = String::new();
    nums.push(num_chars.0);
    nums.push(num_chars.1);
    nums.parse().unwrap()
}

fn get_sum_of_line_numbers(input: Vec<String>) -> i32 {
    let mut current_sum = 0;
    for item in input{
        current_sum = current_sum + get_number_from_first_and_last(item);
    }
    current_sum
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_read_lines() {
        let lines = read_lines("day1_input");
        assert_eq!(lines[0], "nkzjrdqrmpztpqninetwofour1znnkd");
    }

    #[test]
    fn test_get_first_and_last_digit_from_string() {
        let input_string = "&*^K%^&$7ck6785k7x&*%$J564j786k674j&BU(hteiugfoeyucaefo5euao.d".to_string();
        assert_eq!(get_first_and_last_digit(input_string), ("7".parse::<char>().unwrap(), "5".parse::<char>().unwrap()))
    }

    #[test]
    fn test_get_int_from_line() {
        let input_string = "(*^K^4j675J674jk875kj56.3786784j56EU^Oe,u6i96.9B<#X&*9)".to_string();
        assert_eq!(get_number_from_first_and_last(input_string), 49)
    }

    #[test]
    fn test_get_sum_from_vector_of_lines() {
        let mut input = Vec::new();
        let values = ["1abc2".to_string(), "pqr3stu8vwx".to_string(), "a1b2c3d4e5f".to_string(), "treb7uchet".to_string()];
        input.extend(values);
        assert_eq!(get_sum_of_line_numbers(input), 142)
    }

    #[test]
    fn test_get_day1_part1_answer() {

        assert_eq!(get_sum_of_line_numbers(read_lines("day1_input")), 56506);
    }

}

