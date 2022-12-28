use std::{collections::VecDeque, cmp::Ordering};
use PacketPart::*;
use std::cmp::Ordering::*;

pub fn sum_order_pair_indexes(input: &str) -> usize {
    let packet_parts: Vec<PacketPart> = input.trim()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(parse_line)
        .collect();
    let chunks: Vec<&[PacketPart]> = packet_parts
        .chunks(2)
        .collect();

    let mut index_sum = 0;
    for i in 0..chunks.len() {
        let chunk = chunks[i];
        if chunk[0].cmp(&chunk[1]) == Less {
            println!("Index is ordered {}", i + 1);
            index_sum += i + 1;
        }
    }
    return index_sum;
}

fn parse_line(line: &str) -> PacketPart {
    let mut tokens = tokenize_line(line);
    return recursive_parse(&mut tokens);
}

fn recursive_parse(tokens: &mut VecDeque<String>) -> PacketPart {
    return match tokens.pop_front().as_deref() {
        Some("[") => {
            let mut parts: Vec<PacketPart> = Vec::new();
            while tokens.front() != Some(&"]".to_string()) {
                parts.push(recursive_parse(tokens));
            }
            tokens.pop_front(); // "]"
            return PacketPart::ArrayPart(parts);
        },
        Some(val) => PacketPart::NumPart(val.parse().unwrap()),
        _ => panic!("Cannot handle token")
    }
}

fn tokenize_line(line: &str) -> VecDeque<String> {
    let delimiters = ['[',']', ','];
    let mut tokens: VecDeque<String> = VecDeque::new();
    for t in line.trim().split_inclusive(delimiters) {
        match t.chars().last() {
            Some(d) if delimiters.contains(&d) => {
                if t.len() > 1 {
                    tokens.push_back(t[..t.len() - 1].to_string());
                }
                if d != ',' {
                    tokens.push_back(d.to_string());
                }
            },
            _ => {
                tokens.push_back(t.to_string());
            },
        };
    };
    return tokens;
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum PacketPart {
    NumPart(i32),
    ArrayPart(Vec<PacketPart>),
}

impl PacketPart {
    fn from(vals: &Vec<i32>) -> PacketPart {
        let parts: Vec<PacketPart> = vals.iter().map(|v| PacketPart::NumPart(*v)).collect();
        return PacketPart::ArrayPart(parts);
    }
}

impl Ord for PacketPart {
    fn cmp(&self, other: &Self) -> Ordering {
        // println!("Comparing {:?} to {:?}", self, other);
        return match (self, other) {
            (NumPart(left_val), NumPart(right_val)) => left_val.cmp(right_val),
            (NumPart(_), ArrayPart(_)) => ArrayPart(vec![self.to_owned()]).cmp(other),
            (ArrayPart(_), NumPart(_)) => self.cmp(&ArrayPart(vec![other.to_owned()])),
            (ArrayPart(left_items), ArrayPart(right_items)) => {
                for i in 0..left_items.len() {
                    if right_items.len() <= i {
                        // println!("Out of right items: {:?} {:?}", right_items.len(), i);
                        return Greater;
                    }
                    let sub_compare = left_items[i].cmp(&right_items[i]);
                    if sub_compare != Equal {
                        // println!("Broke out: {:?}", sub_compare);
                        return sub_compare;
                    }
                }
                // println!("Out of left items");
                return Less;
            },
        };
    }
}

impl PartialOrd for PacketPart {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("[1,1,3,1,1]", PacketPart::from(&vec![1, 1, 3, 1, 1]))]
    #[test_case("[[1],[2,3,4]]", ArrayPart(vec![
        PacketPart::from(&vec![1]),
        PacketPart::from(&vec![2, 3, 4])
    ]))]
    #[test_case("[[1],4]", 
        ArrayPart(vec![
            PacketPart::from(&vec![1]),
            NumPart(4)
        ])
    )]
    #[test_case("[9]", PacketPart::from(&vec![9]))]
    #[test_case("[]", ArrayPart(vec![]))]
    #[test_case("[[[]]]", ArrayPart(vec![
        ArrayPart(vec![ArrayPart(vec![])])
    ]))]
    fn test_parse_line(line: &str, expected_part: PacketPart) {
        let result = parse_line(line);
        assert_eq!(result, expected_part);
    }

    #[test_case("[1,1,3,1,1]", "[1,1,5,1,1]", Less)]
    #[test_case("[[1],[2,3,4]]", "[[1],4]", Less)]
    #[test_case("[9]", "[[8,7,6]]", Greater)]
    #[test_case("[[4,4],4,4]", "[[4,4],4,4,4]", Less)]
    #[test_case("[7,7,7,7]", "[7,7,7]", Greater)]
    #[test_case("[]", "[3]", Less)]
    #[test_case("[[[]]]", "[[]]", Greater)]
    #[test_case("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]", Greater)]
    fn test_pair_in_correct_order(a: &str, b: &str, expected_ordering: Ordering) {
        let part_a = parse_line(a);
        let part_b = parse_line(b);
        assert_eq!(&part_a.cmp(&part_b), &expected_ordering);
    }

    #[test]
    fn test_sum_order_pair_indexes() {
        let input = r#"
            [1,1,3,1,1]
            [1,1,5,1,1]
            
            [[1],[2,3,4]]
            [[1],4]
            
            [9]
            [[8,7,6]]
            
            [[4,4],4,4]
            [[4,4],4,4,4]
            
            [7,7,7,7]
            [7,7,7]
            
            []
            [3]
            
            [[[]]]
            [[]]
            
            [1,[2,[3,[4,[5,6,7]]]],8,9]
            [1,[2,[3,[4,[5,6,0]]]],8,9]
        "#;
        let result = sum_order_pair_indexes(input);
        assert_eq!(result, 13);
    }
}