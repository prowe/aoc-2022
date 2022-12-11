use core::slice::Iter;

pub fn count_visible_trees(input: &str) -> u32 {
    let forest = parse_string_to_forest(input);
    let mut tree_count = 0;
    for row in 0..forest.len() {
        for col in 0..forest[row].len() {
            if is_tree_visible(row, col, &forest) {
                tree_count += 1;
            }
        }
    }
    return tree_count;
}

pub fn calc_max_senic_score(input: &str) -> u32 {
    let forest = parse_string_to_forest(input);
    let mut max = 0;
    for row in 0..forest.len() {
        for col in 0..forest[row].len() {
            let score = senic_score(row, col, &forest);
            if score > max {
                max = score;
            }
        }
    }
    return max;
}



fn senic_score(row: usize, col: usize, forest: &Vec<Vec<u8>>) -> u32 {
    let height = forest[row][col];
    let calc_viewing_distance = |run: &Vec<u8>| -> usize {
        for i in 0..run.len() {
            if run[i] >= height {
                return i + 1;
            }
        }
        return run.len();
    };
    let score_to_left = calc_viewing_distance(&forest[row][..col].iter().rev().cloned().collect());
    let score_to_right = calc_viewing_distance(&forest[row][col+1..].to_vec());

    let vert_slice: Vec<u8> = forest.iter().map(|row| row[col]).collect();
    let score_to_top = calc_viewing_distance(&vert_slice[..row].iter().rev().cloned().collect());
    let score_to_bottom = calc_viewing_distance(&vert_slice[row+1..].to_vec());

    println!("{:?} {:?} {:?} {:?}", score_to_left, score_to_right, score_to_top, score_to_bottom);
    return (score_to_left * score_to_right * score_to_top * score_to_bottom) as u32;
}

fn is_tree_visible(row: usize, col: usize, forest: &Vec<Vec<u8>>) -> bool {
    let height = forest[row][col];
    let shorter = |val: &u8| (val.to_owned() < height);

    let vis_from_left = forest[row][..col].into_iter().all(shorter);
    let vis_from_right = forest[row][col+1..].into_iter().all(shorter);

    let vert_slice: Vec<u8> = forest.iter().map(|row| row[col]).collect();
    let vis_from_top = vert_slice[..row].into_iter().all(shorter);
    let vis_from_bottom = vert_slice[row+1..].into_iter().all(shorter);

    return vis_from_left || vis_from_right || vis_from_top || vis_from_bottom;
}

fn parse_string_to_forest(input: &str) -> Vec<Vec<u8>> {
    let forest: Vec<Vec<u8>> = input
        .trim()
        .lines()
        .map(parse_string_to_tree_row)
        .collect();
    return forest;
}

fn parse_string_to_tree_row(row: &str) -> Vec<u8> {
    return row.trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(1, 1, true)]
    #[test_case(1, 2, true)]
    #[test_case(1, 3, false)]
    #[test_case(2, 1, true)]
    #[test_case(2, 2, false)]
    #[test_case(2, 3, true)]
    #[test_case(3, 1, false)]
    #[test_case(3, 2, true)]
    #[test_case(3, 3, false)]
    fn test_is_visible(row: usize, col: usize, expected_visible: bool) {
        let input = r#"
            30373
            25512
            65332
            33549
            35390
        "#;
        let forest = parse_string_to_forest(input);
        let is_visible = is_tree_visible(row, col, &forest);
        assert_eq!(is_visible, expected_visible);
    }

    #[test]
    fn test_count_visible_trees() {
        let input = r#"
            30373
            25512
            65332
            33549
            35390
        "#;
        let tree_count = count_visible_trees(input);
        assert_eq!(tree_count, 21);
    }

    #[test_case(1, 2, 4)]
    #[test_case(3, 2, 8)]
    fn test_senic_score(row: usize, col: usize, expected_score: u32) {
        let input = r#"
            30373
            25512
            65332
            33549
            35390
        "#;
        let forest = parse_string_to_forest(input);
        let score = senic_score(row, col, &forest);
        assert_eq!(score, expected_score);
    }
}



