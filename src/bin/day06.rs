use aoc::*;

type State = [i32; 9];

fn parse_input(input: String) -> Vec<i32> {
    return input
        .trim()
        .split(",")
        .map(str::trim)
        .map(str::parse)
        .map(Result::unwrap)
        .collect();
}

fn get_state(input: Vec<i32>) -> State {
    let mut state = [0; 9];
    for num in input {
        state[num as usize] += 1;
    }
    return state;
}

fn pass_day(state: State) -> State {
    return [
        state[1],
        state[2],
        state[3],
        state[4],
        state[5],
        state[6],
        state[7] + state[0],
        state[8],
        state[0],
    ];
}

fn pass_days(state: State, days: i32) -> State {
    let mut state = state;
    for _ in 0..days {
        state = pass_day(state);
    }
    return state;
}

fn part1(input: String) -> i32 {
    let mut state = get_state(parse_input(input));
    state = pass_days(state, 80);
    return state.iter().sum();
}

fn main() {
    run(part1, missing);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = "3,4,3,1,2";

    #[test]
    fn example_parse() {
        assert_eq!(parse_input(EXAMPLE_INPUT.to_string()), vec![3, 4, 3, 1, 2]);
    }

    #[test]
    fn example_pass_days() {
        assert_eq!(
            pass_days([0, 1, 2, 1, 0, 0, 0, 0, 0], 1),
            [1, 2, 1, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(
            pass_days([0, 1, 2, 1, 0, 0, 0, 0, 0], 2),
            [2, 1, 0, 0, 0, 0, 1, 0, 1]
        );
        assert_eq!(
            pass_days([0, 1, 2, 1, 0, 0, 0, 0, 0], 3),
            [1, 0, 0, 0, 0, 1, 2, 1, 2]
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(part1(EXAMPLE_INPUT.to_string()), 5934);
    }
}