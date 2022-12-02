pub fn rock_paper_scissors(input: String) -> u32 {
    let score: u32 = input.split('\n')
        .filter_map(input_line_to_round)
        .map(|r| r.score())
        .sum();
    return score;
}

fn input_line_to_round(line: &str) -> Option<Round> {
    let splitted: Vec<&str> = line.trim().split(' ').collect();
    return Some(Round {
        my_throw: letter_to_throw(splitted[0])?,
        their_throw: letter_to_throw(splitted[1])?,
    });
}

fn letter_to_throw(letter: &str) -> Option<Throw> {
    return match letter.trim() {
        "A" => Some(Throw::ROCK),
        "B" => Some(Throw::PAPER),
        "C" => Some(Throw::SCISSORS),
        "X" => Some(Throw::ROCK),
        "Y" => Some(Throw::PAPER),
        "Z" => Some(Throw::SCISSORS),
        _ => {
            println!("Invalid letter: {}", letter);
            return None;
        }
    };
}

#[derive(Debug)]
struct Round {
    my_throw: Throw,
    their_throw: Throw
}

impl Round {
    fn score(&self) -> u32 {
        let win = 6;
        let lose = 0;
        let draw = 3;
        let outcome = match self.my_throw {
            Throw::ROCK => match self.their_throw {
                Throw::ROCK => draw,
                Throw::PAPER => lose,
                Throw::SCISSORS => win,
            },
            Throw::PAPER => match self.their_throw {
                Throw::ROCK => win,
                Throw::PAPER => draw,
                Throw::SCISSORS => lose,
            },
            Throw::SCISSORS => match self.their_throw {
                Throw::ROCK => lose,
                Throw::PAPER => win,
                Throw::SCISSORS => draw,
            }
        };
        let bonus = match self.my_throw {
            Throw::ROCK => 1,
            Throw::PAPER => 2,
            Throw::SCISSORS => 3,
        };
        return outcome + bonus;
    }
}

#[derive(Debug)]
enum Throw {
    ROCK,
    PAPER,
    SCISSORS
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_sample_input() {
        let input = r#"
            A Y
            B X
            C Z
        "#;
        let score = rock_paper_scissors(input.to_string());
        assert_eq!(score, 15);
    }

    #[test_case(Throw::ROCK, Throw::ROCK, 3, 1)]
    #[test_case(Throw::ROCK, Throw::PAPER, 0, 1)]
    #[test_case(Throw::ROCK, Throw::SCISSORS, 6, 1)]
    #[test_case(Throw::PAPER, Throw::ROCK, 6, 2)]
    #[test_case(Throw::PAPER, Throw::PAPER, 3, 2)]
    #[test_case(Throw::PAPER, Throw::SCISSORS, 0, 2)]
    #[test_case(Throw::SCISSORS, Throw::ROCK, 0, 3)]
    #[test_case(Throw::SCISSORS, Throw::PAPER, 6, 3)]
    #[test_case(Throw::SCISSORS, Throw::SCISSORS, 3, 3)]
    fn test_score_calculation(my_throw: Throw, their_throw: Throw, expected_outcome: u32, expected_bonus: u32) {
        let round = Round {
            my_throw: my_throw,
            their_throw: their_throw,
        };
        let expected_score = expected_outcome + expected_bonus;
        assert_eq!(round.score(), expected_score);
    }
}