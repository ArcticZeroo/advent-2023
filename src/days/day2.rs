use std::cmp::max;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use crate::models::solution::Solution;

lazy_static!(
    static ref GAME_REGEX: Regex = Regex::new(r"^Game (?<id>\d+): (?<rounds>.+)$").unwrap();
);

struct ColorCount {
    blue: i32,
    green: i32,
    red: i32,
}

impl ColorCount {
    pub fn can_make(&self, other: &ColorCount) -> bool {
        self.blue >= other.blue
            && self.green >= other.green
            && self.red >= other.red
    }
}

pub struct Day2 {
    games_by_id: HashMap<i32, Vec<ColorCount>>,
}

fn parse_game(line: &str) -> (i32, Vec<ColorCount>) {
    let captures = GAME_REGEX.captures(line).expect("Could not parse game line");

    let id = captures["id"].parse::<i32>().expect("Could not parse game id");
    let rounds = captures["rounds"]
        .split(";")
        .map(|round_str| {
            let mut color_count = ColorCount { blue: 0, green: 0, red: 0 };

            for color_count_str in round_str.trim().split(", ") {
                let color_count_parts = color_count_str.trim().split(" ").collect::<Vec<&str>>();
                let count = color_count_parts[0].parse::<i32>().expect("Could not parse color count");
                let color = color_count_parts[1];

                match color {
                    "blue" => color_count.blue += count,
                    "green" => color_count.green += count,
                    "red" => color_count.red += count,
                    _ => panic!("Unknown color {}", color)
                }
            }

            color_count
        })
        .collect::<Vec<ColorCount>>();

    (id, rounds)
}

impl Solution for Day2 {
    fn day() -> i32 {
        2
    }

    fn new(input: &str) -> Self {
        Day2 {
            games_by_id: input
                .lines()
                .map(parse_game)
                .collect::<HashMap<i32, Vec<ColorCount>>>()
        }
    }

    fn part1(&self) -> String {
        let bag_counts = ColorCount {
            red: 12,
            green: 13,
            blue: 14,
        };

        self.games_by_id.iter()
            .filter_map(|(id, rounds)| {
                if rounds.iter().all(|round| bag_counts.can_make(round)) {
                    Some(id)
                } else {
                    None
                }
            })
            .sum::<i32>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.games_by_id
            .iter()
            .map(|(_, rounds)| {
                let mut required_counts = ColorCount { blue: 0, green: 0, red: 0 };
                for round in rounds {
                    required_counts.blue = required_counts.blue.max(round.blue);
                    required_counts.green = required_counts.green.max(round.green);
                    required_counts.red = required_counts.red.max(round.red);
                }
                required_counts.blue * required_counts.green * required_counts.red
            })
            .sum::<i32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let solution = Day2::new(input);
        assert_eq!(solution.part1(), "8");
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let solution = Day2::new(input);
        assert_eq!(solution.part2(), "2286");
    }
}
