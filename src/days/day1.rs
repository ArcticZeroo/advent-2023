use crate::models::solution::Solution;
use crate::util::input::input_to_lines;

pub struct Day1 {
    lines: Vec<String>,
}

fn chars_to_num(chars: &Vec<char>) -> i32 {
    let a = chars.first().ok_or_else(|| format!("Could not get first numeric char in {:?}", chars)).unwrap();
    let b = chars.last().ok_or_else(|| format!("Could not get last numeric char in {:?}", chars)).unwrap();

    format!("{}{}", a, b).parse::<i32>().expect("Could not parse number")
}

fn substr_to_num(substr: &str) -> Option<char> {
    match substr {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None
    }
}

fn is_possible_num_start(char: char) -> bool {
    match char {
        'o' | 't' | 'f' | 's' | 'e' | 'n' => true,
        _ => false
    }
}

fn line_to_number(line: &str) -> i32 {
    let mut chars = vec![];

    let mut i = 0;
    while i < line.len() {
        let char = line.chars().nth(i).unwrap();

        if char.is_numeric() {
            chars.push(char);
        } else if is_possible_num_start(char) {
            for j in i + 1..=line.len() {
                let substr = &line[i..j];
                let substr_num = substr_to_num(substr);
                if let Some(num) = substr_num {
                    chars.push(num);
                    break;
                }
            }
        }

        i += 1;
    }

    chars_to_num(&chars)
}

impl Solution for Day1 {
    fn day() -> i32 {
        1
    }

    fn new(input: &str) -> Self {
        Day1 {
            lines: input_to_lines(input),
        }
    }

    fn part1(&self) -> String {
        self.lines
            .iter()
            .map(|s| {
                let numeric_chars = s
                    .chars()
                    .filter(|c| c.is_numeric())
                    .collect::<Vec<char>>();

                chars_to_num(&numeric_chars)
            })
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.lines
            .iter()
            .map(|s| line_to_number(s))
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let solution = Day1::new(input);
        assert_eq!(solution.part1(), "142");
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let solution = Day1::new(input);
        assert_eq!(solution.part2(), "281");
    }
}