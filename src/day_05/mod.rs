use std::vec;

use regex::{self, Regex};

pub fn calculate_crane_moves(move_text: &str, mut stacks: Vec<Vec<char>>) -> String {
    let moves = parse_to_moves(move_text);

    log_stacks(&stacks);
    println!("Processing {:?} moves", moves.len());

    let mut move_count = 0;
    for m in moves {
        move_count += 1;
        if stacks[m.source].len() < m.count {
            panic!("Stack is empty for move: {:?} {:?}", move_count, m);
        }
        for _i in 0..m.count {
            let ch = stacks[m.source].pop().unwrap();
            stacks[m.target].push(ch);
        }
    }
    
    log_stacks(&stacks);
    return get_stack_tops(stacks);
}

pub fn calculate_crate_mover_9001(move_text: &str, mut stacks: Vec<Vec<char>>) -> String {
    let moves = parse_to_moves(move_text);

    for m in moves {
        let cut_index = stacks[m.source].len() - m.count;
        let mut pick_up_crates = stacks[m.source].split_off(cut_index);
        stacks[m.target].append(&mut pick_up_crates);        
    }

    return get_stack_tops(stacks);
}

fn parse_to_moves(move_text: &str) -> Vec<Move> {
    let moves: Vec<Move> = move_text.trim()
        .lines()
        .map(parse_to_move)
        .collect();
    return moves;
}

fn get_stack_tops(stacks: Vec<Vec<char>>) -> String {
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
    let ordered = vec![
        "GJWRFTZ",
        "MWG",
        "GHNJ",
        "WNCRJ",
        "MVQGBSFW",
        "CWVDTRS",
        "VGZDCNBH",
        "CGMNJS",
        "LDJCWNPG",
    ];
    return ordered.iter()
        .map(|s| s.chars().rev().collect())
        .collect();
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

    #[test]
    fn test_improved_crane() {
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
        let resulting_top = calculate_crate_mover_9001(input.trim(), stacks);
        assert_eq!(resulting_top, "MCD");
    }
}