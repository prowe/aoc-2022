use std::{iter, collections::VecDeque};

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
    for t in line.split_inclusive(delimiters) {
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

#[derive(PartialEq, Eq, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use super::PacketPart::*;
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
}