use std::vec;

use regex::{self, Regex};

pub fn calculate_crane_moves(move_text: &str, mut stacks: Vec<Vec<char>>) -> String {
    let moves: Vec<Move> = move_text.trim()
        .lines()
        .map(parse_to_move)
        .collect();

    log_stacks(&stacks);
    println!("Processing {:?} moves", moves.len());

    for m in moves {
        for _i in 0..m.count {
            let ch = stacks[m.source].pop().unwrap();
            stacks[m.target].push(ch);
        }
    }
    
    log_stacks(&stacks);
    let tops: String = stacks
        .iter()
        .filter_map(|s| s.last())
        .cloned()
        .collect();
    // println!("tops: {:?}", tops);
    return tops;
}

fn log_stacks(stacks: &Vec<Vec<char>>) {
    let mut s_index = 0;
    for s in stacks.clone() {
        println!("{:?}: {:?}", s_index, s);
        s_index += 1;
    }
}

pub fn build_stacks() -> Vec<Vec<char>> {
    return vec![
        "ZTFRQJG".chars().collect(),
        "GWM".chars().collect(),
        "JNHG".chars().collect(),
        "JRCNW".chars().collect(),
        "WFSBGQVM".chars().collect(),
        "SRTDVWC".chars().collect(),
        "HBNCDZGV".chars().collect(),
        "SJNMGC".chars().collect(),
        "GPNWCJDL".chars().collect(),
    ];
}

fn parse_to_move(line: &str) -> Move {
    let move_regex = Regex::new(r".*move (\d+) from (\d+) to (\d+)").unwrap();
    let captures = move_regex.captures(line).unwrap();
    let m = Move {
        count: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
        source: captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
        target: captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
    };
    // println!("Parsed move: {:?}", m);
    return m;
}

#[derive(Debug)]
struct Move {
    source: usize,
    target: usize,
    count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_sample_input() {
        let stacks = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];
        let input = r#"
            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "#;
        let resulting_top = calculate_crane_moves(input.trim(), stacks);
        assert_eq!(resulting_top, "CMZ");
    }
}