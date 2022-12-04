use std::collections::HashSet;

pub fn calc_total_of_high_priority(input: &str) -> u32 {
    let total = input
        .lines()
        .map(|l| l.trim())
        .map(calc_line_total)
        .sum();
    return total;
}

fn calc_line_total(line: &str) -> u32 {
    let compartment_size = line.len() / 2;
    println!("Line: {}, len: {}, size: {}", line, line.len(), compartment_size);
    let left: HashSet<char> = HashSet::from_iter(line.chars().take(compartment_size));
    let right: HashSet<char> = HashSet::from_iter(line.chars().skip(compartment_size));
    println!("Left: {:?}, right: {:?}", left, right);
    return left
        .intersection(&right)
        .map(letter_to_value)
        .max()
        .unwrap_or(0);
}

fn letter_to_value(letter: &char) -> u32 {
    if letter.is_lowercase() {
        return (*letter as u32) - ('a' as u32) + 1;
    }
    return (*letter as u32) - ('A' as u32) + 27;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_part1_sample() {
        let input= r#"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "#;

        let total = calc_total_of_high_priority(input);
        assert_eq!(total, 157);
    }

    #[test_case('a', 1)]
    #[test_case('z', 26)]
    #[test_case('A', 27)]
    #[test_case('Z', 52)]
    fn test_letter_to_value(letter: char, expected_value: u32) {
        let val = letter_to_value(&letter);
        assert_eq!(val, expected_value);
    }
}