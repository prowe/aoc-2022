use std::ops::Range;

pub fn calculate_total_signal(input: &str) -> i32 {
    let expanded_steps = parse_input_into_expanded_steps(input);
    let steps_to_sum: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let strengths: Vec<i32> = steps_to_sum
        .iter()
        .cloned()
        .map(|step| compute_value_at_step(&expanded_steps, step))
        .collect();

    println!("Total strength: {:?} {:?}", expanded_steps.len(), expanded_steps.iter().sum::<i32>());
    println!("Strengths: {:?}", strengths);

    return strengths.iter().sum();
}

pub fn parse_steps_into_ascii_art(input: &str) -> String {
    let expanded_steps = parse_input_into_expanded_steps(input);
    let register_values = fold_steps_into_values(&expanded_steps);
    
    let mut screen: Vec<char> = Vec::new();
    for clock in 0..240 {
        let col = clock_to_col(clock);
        let reg_value = if clock == 0 { 1 } else { register_values[(clock - 0) as usize] };
        let sprite_pos = register_value_to_sprite_slice(reg_value);
        screen.push(if sprite_pos.contains(&col) { '#' } else { '.' });
        println!("{:?}: last val: {:?} sprite_pos: {:?}, col: {:?}", clock, reg_value, sprite_pos, col);
    }
    return screen
        .as_slice()
        .chunks(40)
        .map(|row| row.into_iter().collect::<String>() + "\n")
        .collect::<String>()
        .trim()
        .to_string();
}

fn fold_steps_into_values(expanded_steps: &Vec<i32>) -> Vec<i32> {
    let mut folded = expanded_steps.iter()
        .fold(vec![1], |mut acc, diff| {
            let tail = acc.last().unwrap();
            acc.push(tail + diff);
            return acc;
        });
    folded.pop();
    return folded;
}

fn clock_to_col(clock: i32) -> i32 {
    return clock % 40;
}

fn parse_input_into_expanded_steps(input: &str) -> Vec<i32> {
    let expanded_steps: Vec<i32> = input
        .lines()
        .map(str::trim)
        .flat_map(convert_line_to_steps)
        .collect();
    return expanded_steps;
}

fn convert_line_to_steps(line: &str) -> Vec<i32> {
    if line == "noop" {
        return vec![0];
    }
    let value: i32 = line.split(' ').last().unwrap().parse().unwrap();
    return vec![0, value];
}

fn compute_value_at_step(expanded_steps: &Vec<i32>, step: usize) -> i32 {
    let slice_end = step - 1;
    let sum: i32 = 1 + expanded_steps[..slice_end].into_iter().sum::<i32>();
    let value = sum * (step as i32);
    println!("Computing value: {:?}, slice-end: {:?} sum: {:?} value: {:?}", step, slice_end, sum, value);
    return value;
}

fn register_value_to_sprite_slice(value: i32) -> Range<i32> {
    return value-1..value+2;
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};
    use test_case::test_case;
    use super::*;

    #[test]
    fn test_sample_input_pt1() {
        let mut file = File::open("src/day_10/sample.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let signal = calculate_total_signal(&contents);
        assert_eq!(signal, 13140);
    }

    #[test]
    fn test_input_is_parsed_into_steps() {
        let input = r#"
            addx 15
            addx -11
            noop
        "#;
        let expanded_steps = parse_input_into_expanded_steps(&input.trim());
        assert_eq!(expanded_steps, vec![0, 15, 0, -11, 0]);
    }

    #[test_case(vec![0, 15, 0, -11, 0], 1, 1)]
    #[test_case(vec![0, 15, 0, -11, 0], 2, 2)]
    #[test_case(vec![0, 15, 0, -11, 0], 3, 48)]
    #[test_case(vec![0, 15, 0, -11, 0], 4, 64)]
    #[test_case(vec![0, 15, 0, -11, 0], 5, 25)]
    fn test_compute_value_at_step(expanded_steps: Vec<i32>, step: usize, expected_value: i32) {
        assert_eq!(compute_value_at_step(&expanded_steps, step), expected_value);
    }

    #[test]
    fn test_fold_steps_into_values() {
        let values = fold_steps_into_values(&vec![0, 15, 0, -11, 0]);
        assert_eq!(values, vec![1, 1, 16, 16, 5]);
    }

    #[test_case(0, 0)]
    #[test_case(39, 39)]
    #[test_case(40, 0)]
    #[test_case(79, 39)]
    #[test_case(80, 0)]
    #[test_case(119, 39)]
    #[test_case(120, 0)]
    #[test_case(159, 39)]
    #[test_case(160, 0)]
    #[test_case(199, 39)]
    #[test_case(200, 0)]
    #[test_case(239, 39)]
    fn test_clock_to_col(clock: i32, expected_col: i32) {
        let col = clock_to_col(clock);
        assert_eq!(col, expected_col)
    }

    #[test_case(1, 0..3)]
    #[test_case(5, 4..7)]
    fn test_register_value_to_sprite_slice(value: i32, expected_slice: Range<i32>) {
        assert_eq!(register_value_to_sprite_slice(value), expected_slice);
    }

    #[test]
    fn test_ascii_art() {
        let mut file = File::open("src/day_10/sample.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let art = parse_steps_into_ascii_art(&contents);
        let expected_art = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
        "#.trim();
        assert_eq!(art, expected_art);
    }
}