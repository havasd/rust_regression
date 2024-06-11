use std::ops::Sub;

pub fn p1(input: &str) -> i64 {
    let mut result = 0;
    for line in input.lines() {
        let numbers =
            parse_line_into_sequence_of_numbers(line).collect::<Vec<_>>();
            result += predict_next_value(numbers);
    }
    result
}

fn predict_next_value(numbers: Vec<i64>) -> i64 {
    let mut sequence = numbers;
    let mut stack_of_lastvalues = Vec::new();
    while sequence.iter().any(|v| *v != 0) {
        let lastvalue = *sequence.last().unwrap();
        stack_of_lastvalues.push(lastvalue);
        if let Some(differences) =
            generate_differences_and_values(sequence.into_iter())
        {
            sequence = differences.collect();
        } else {
            break;
        }
    }
     stack_of_lastvalues.into_iter().sum()
}

#[allow(unused_variables)]
pub fn p2(input: &str) -> u64 {
    todo!("p2")
}

fn parse_line_into_sequence_of_numbers(
    line: &str,
) -> impl Iterator<Item = i64> + '_ {
    line.split_whitespace().map(|number_string| {
        number_string.parse::<i64>().expect("parse error in line")
    })
}

fn generate_differences_and_values<T: Copy + Sub<Output = T>>(
    mut sequence: impl Iterator<Item = T>,
) -> Option<impl Iterator<Item = T>> {
    sequence.next().and_then(|v1| {
        Some(sequence.scan(v1, |acc, v| {
            let difference = v - *acc;
            *acc = v;
            Some(difference)
        }))
    })
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::{generate_differences_and_values, p1, parse_line_into_sequence_of_numbers, predict_next_value};

    #[test]
    fn test_predict_next_value() {
        let input = parse_line_into_sequence_of_numbers("1 3 6 10 15 21").collect();
        let result = predict_next_value(input);
        assert_eq!(result, 28);
    }

    #[test]
    fn test_predict_next_value2() {
        let input = parse_line_into_sequence_of_numbers("10 13 16 21 30 45").collect();
        let result = predict_next_value(input);
        assert_eq!(result, 68);
    }

    #[test]
    fn test_predict_next_value3() {
        let input = parse_line_into_sequence_of_numbers("0 3 6 9 12 15").collect();
        let result = predict_next_value(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_generate_differences_and_values() {
        let input = parse_line_into_sequence_of_numbers("7 15 32 57 98 176 332 653 1352 2972 6842 16010 37046 83402 181521 381725 777249 1536841 2959392 5563435 10230470")
            .collect::<Vec<_>>();
        let len = input.len();
        if let Some(result) = generate_differences_and_values(input.into_iter()) {
            let result = result.collect::<Vec<_>>();
            println!("{:?}", result);
            assert_eq!(result.len(), len -1);
        } else {
            assert!(false)
        }
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