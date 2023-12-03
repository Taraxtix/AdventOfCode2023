const FILENAME: &str = "src/input.txt";

fn get_digit(line: &str) -> u32 {
    let mut res = 0;
    for c in line.chars() {
        if c.is_numeric() { res = c.to_digit(10).unwrap() * 10; break; }
    }
    for c in line.chars().rev() {
        if c.is_numeric() { res += c.to_digit(10).unwrap(); break; }
    }
    res
}

fn main() {
    let file = std::fs::read_to_string(FILENAME).unwrap();
    let part1_lines = file.lines();
    let part1_digits = part1_lines.map(|line| get_digit(line));

    let part2_file = file.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "4")
        .replace("five", "5e")
        .replace("six", "6")
        .replace("seven", "7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    let part2_lines = part2_file.lines();
    let part2_digits = part2_lines.map(|line| get_digit(line));

    println!("Part1: {}", part1_digits.sum::<u32>());
    println!("Part2: {}", part2_digits.sum::<u32>());
}