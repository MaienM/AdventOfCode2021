use aoc::runner::*;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Down,
    Up,
}

type Instruction = (Direction, u32);

fn parse_input(input: String) -> Vec<Instruction> {
    let lines = input.trim().split("\n").map(str::trim);
    return lines
        .map(|line| {
            let mut parts = line.splitn(2, " ");
            let direction = parts
                .next()
                .unwrap_or_else(|| panic!("Unable to get direction from line {}.", line));
            let direction = match direction {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => panic!("Invalid line {}.", line),
            };
            let distance = parts
                .next()
                .unwrap_or_else(|| panic!("Unable to get distance from line {}.", line))
                .parse::<u32>()
                .unwrap_or_else(|_| panic!("Unable to parse distance from line {}.", line));
            return (direction, distance);
        })
        .collect();
}

pub fn part1(input: String) -> i64 {
    let instructions = parse_input(input);
    let mut hpos = 0;
    let mut vpos = 0;
    for instruction in instructions {
        let (direction, distance) = instruction;
        match direction {
            Direction::Forward => hpos += distance,
            Direction::Down => vpos += distance,
            Direction::Up => vpos -= distance,
        }
    }
    return (hpos * vpos).into();
}

pub fn part2(input: String) -> i64 {
    let instructions = parse_input(input);
    let mut aim = 0;
    let mut hpos = 0;
    let mut vpos = 0;
    for instruction in instructions {
        let (direction, distance) = instruction;
        match direction {
            Direction::Forward => {
                hpos += distance;
                vpos += distance * aim;
            }
            Direction::Down => aim += distance,
            Direction::Up => aim -= distance,
        }
    }
    return (hpos * vpos).into();
}

fn main() {
    run(part1, part2);
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const EXAMPLE_INPUT: &'static str = "
        forward 5
        down 5
        forward 8
        up 3
        down 8
        forward 2
    ";

    #[test]
    fn example_parse() {
        let actual = parse_input(EXAMPLE_INPUT.to_string());
        let expected = vec![
            (Direction::Forward, 5),
            (Direction::Down, 5),
            (Direction::Forward, 8),
            (Direction::Up, 3),
            (Direction::Down, 8),
            (Direction::Forward, 2),
        ];
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 150);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(EXAMPLE_INPUT.to_string()), 900);
    }
}
