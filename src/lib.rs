use std::mem::replace;

pub fn p1(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line_into_sequence_of_numbers)
        .map(predict_next_value)
        .sum::<i64>()
}

pub fn p2(input: &str) -> i64 {
    input
        .lines()
        .map(parse_line_into_sequence_of_numbers)
        .rev()
        .map(predict_next_value)
        .sum::<i64>()
}

fn parse_line_into_sequence_of_numbers(
    line: &str,
) -> impl DoubleEndedIterator<Item = i64> + '_ {
    line.split_whitespace().map(|number_string| {
        number_string.parse::<i64>().expect("parse error in line")
    })
}

fn predict_next_value(numbers: impl DoubleEndedIterator<Item = i64>) -> i64 {
    let mut sequence = numbers.rev().collect::<Vec<_>>();

    // Although it looks like a fold, it is faster imperative
    let mut result = 0;
    while sequence.iter().any(|&v| v != 0) {
        result += *sequence.first().unwrap();
        sequence = generate_differences(sequence);
    }
    result
}

fn generate_differences(sequence: Vec<i64>) -> Vec<i64> {
    let mut sequence = sequence.into_iter();
    if let Some(result) = sequence.next().map(|firstvalue| {
        sequence.scan(firstvalue, |acc, v| {
            Some(replace(acc, v) - v)
        })
    }) {
        Vec::from_iter(result)
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::{
        generate_differences, p1, parse_line_into_sequence_of_numbers,
        predict_next_value,
    };

    #[test]
    fn test_predict_next_value() {
        let result = predict_next_value(
            &mut parse_line_into_sequence_of_numbers("1 3 6 10 15 21"),
        );
        assert_eq!(result, 28);
    }

    #[test]
    fn test_predict_next_value2() {
        let result = predict_next_value(
            &mut parse_line_into_sequence_of_numbers("10 13 16 21 30 45"),
        );
        assert_eq!(result, 68);
    }

    #[test]
    fn test_predict_next_value4() {
        let result = predict_next_value(
            &mut parse_line_into_sequence_of_numbers("10 13 16 21 30 45").rev(),
        );
        assert_eq!(result, 5);
    }

    #[test]
    fn test_predict_next_value3() {
        let result = predict_next_value(
            &mut parse_line_into_sequence_of_numbers("0 3 6 9 12 15"),
        );
        assert_eq!(result, 18);
    }

    #[test]
    fn test_generate_differences_and_values() {
        let input = parse_line_into_sequence_of_numbers("7 15 32 57 98 176 332 653 1352 2972 6842 16010 37046 83402 181521 381725 777249 1536841 2959392 5563435 10230470")
            .collect::<Vec<_>>();
        let len = input.len();
        let result = generate_differences(input);
        assert_eq!(result.len(), len - 1);
    }

    #[test]
    fn test_p1_sample() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let result = p1(input);
        assert_eq!(result, 114);
    }

    #[test]
    fn test_p1_real() {
        let mut f = File::open("input.txt").expect("can't open file");
        let mut buf = String::new();
        f.read_to_string(&mut buf).expect("can't read file");
        let result = p1(&buf);
        assert_eq!(result, 2043677056);
    }
}
