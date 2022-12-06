use std::collections::HashSet;

pub fn calculate_first_marker(data_stream: &str) -> usize {
    return calculate_index_of_unique_run(data_stream, 4);
}

pub fn calculate_start_of_message_index(data_stream: &str) -> usize {
    return calculate_index_of_unique_run(data_stream, 14);
}

pub fn calculate_index_of_unique_run(data_stream: &str, length: usize) -> usize {
    for i in 0..data_stream.len() {
        if i < length {
            continue;
        }
        let history_slice = &data_stream[(i-length)..i];
        let x: HashSet<char> = HashSet::from_iter(history_slice.chars());
        if x.len() == length {
            return i;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_calculate_first_marker(data_stream: &str, expected_pos: usize) {
        let index = calculate_first_marker(data_stream);
        assert_eq!(index, expected_pos);
    }

    #[test_case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[test_case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[test_case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[test_case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[test_case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_calculate_start_of_message_index(data_stream: &str, expected_pos: usize) {
        let index = calculate_start_of_message_index(data_stream);
        assert_eq!(index, expected_pos);
    }
}