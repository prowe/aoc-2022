use std::ops::Range;

pub fn count_containing_pairs(input: &str) -> usize {
    let pairs = input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(parse_to_ranges)
        .filter(ranges_are_contained)
        .count();

    return pairs;
}

pub fn count_overlapping_pairs(input: &str) -> usize {
    return input
        .lines()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(parse_to_ranges)
        .filter(ranges_overlap)
        .count();
}

fn parse_to_ranges(line: &str) -> Vec<Range<u32>> {
    let x = line.split(',').map(parse_range).collect();
    println!("Pair: {:?}", x);
    return x;
}

fn ranges_are_contained(ranges: &Vec<Range<u32>>) -> bool {
    let a = ranges[0].to_owned();
    let b = ranges[1].to_owned();
    if a.start <= b.start && a.end >= b.end {
        return true;
    }
    if b.start <= a.start && b.end >= a.end {
        return true;
    }
    return false;
}

fn ranges_overlap(ranges: &Vec<Range<u32>>) -> bool {
    let a = ranges[0].to_owned();
    let b = ranges[1].to_owned();
    if a.contains(&b.start) || b.contains(&a.start) {
        return true;
    }
    return false;
}

fn parse_range(value: &str) -> Range<u32> {
    let vals: Vec<&str> = value.split('-').collect();
    return Range {
        start: vals[0].parse().expect("Value not parsable"),
        end: vals[1].parse::<u32>().expect("Value not parsable") + 1,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_sample_input() {
        let input = r#"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "#;
        let count = count_containing_pairs(input);
        assert_eq!(count, 2);
    }

    #[test]
    fn test_any_overlap() {
        let input = r#"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "#;
        let count = count_overlapping_pairs(input);
        assert_eq!(count, 4);
    }

    #[test_case(5..8, 7..10, true)]
    fn test_overlap(a: Range<u32>, b: Range<u32>, expected_overlap: bool) {
        let overlaps = ranges_overlap(&vec![a, b]);
        assert_eq!(overlaps, expected_overlap);
    }
}
