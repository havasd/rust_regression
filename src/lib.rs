use std::ops::{AddAssign, Sub};

pub fn p1(input: &str) -> i64 {
    input.lines().fold(0, |acc, line| {
        predict_next_value(parse_line_into_sequence_of_numbers(line))
            + acc
    })
}

pub fn p2(input: &str) -> i64 {
    input.lines().fold(0, |acc, line| {
        predict_next_value(parse_line_into_sequence_of_numbers(line).rev()) + acc
    })
}

fn parse_line_into_sequence_of_numbers(
    line: &str,
) -> impl DoubleEndedIterator<Item = i64> + '_ {
    line.split_whitespace().map(|number_string| {
        number_string.parse::<i64>().expect("parse error in line")
    })
}

fn predict_next_value<T>(numbers: impl Iterator<Item = T>) -> T
where
    T: Copy + Sub<Output = T> + PartialEq + Default + AddAssign,
{
    let mut sequence = numbers.collect::<Vec<_>>();

    let mut result = T::default();
    while sequence.iter().any(|v| *v != T::default()) {
        result += *sequence.last().unwrap();

        if let Some(differences) =
            generate_differences_and_values(sequence.into_iter())
        {
            sequence = differences.collect();
        } else {
            break;
        }
    }
    result
}

fn generate_differences_and_values<T>(
    mut sequence: impl Iterator<Item = T>,
) -> Option<impl Iterator<Item = T>>
where
    T: Copy + Sub<Output = T>,
{
    sequence.next().map(|v1| {
        sequence.scan(v1, |acc, v| {
            let difference = v - *acc;
            *acc = v;
            Some(difference)
        })
    })
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::{
        generate_differences_and_values, p1,
        parse_line_into_sequence_of_numbers, predict_next_value,
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
        if let Some(result) = generate_differences_and_values(input.into_iter())
        {
            let result = result.collect::<Vec<_>>();
            assert_eq!(result.len(), len - 1);
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
