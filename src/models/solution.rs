pub trait Solution {
    fn day() -> i32;
    fn new(input: &str) -> Self;
    fn part1(&self) -> String;
    fn part2(&self) -> String {
        String::new()
    }
    fn run(&self) {
        println!("Day {}:", Self::day());
        println!("  Part 1: {}", self.part1());
        println!("  Part 2: {}", self.part2());
    }
}

pub fn run_day<T: Solution>() {
    let input = crate::util::input::get_input(T::day());
    let solution = T::new(&input);
    solution.run();
}