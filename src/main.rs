const FILENAME: &str = "src/input.txt";

fn main() {
    let file = std::fs::read_to_string(FILENAME).unwrap();
    let lines = file.lines();
    let part1_digits = lines.map(|line| {
        let mut res = 0;
        for c in line.chars() {
            if c.is_numeric() { res = c.to_digit(10).unwrap() * 10; break; }
        }
        for c in line.chars().rev() {
            if c.is_numeric() { res += c.to_digit(10).unwrap(); break; }
        }
        res
    });
    println!("Part1: {}", part1_digits.sum::<u32>());
}