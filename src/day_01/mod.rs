use regex::{self, Regex};

fn parse_calorie_line(calorie_line: &str) -> u32 {
    match calorie_line.trim().parse() {
        Ok(val) => val,
        Err(_) => 0,
    }
}

fn compute_per_elf_calories(calorie_strings: &str) -> u32 {
    let calorie_split_regex = Regex::new(r"[[:space:]]+").unwrap();
    let cals: u32 = calorie_split_regex
        .split(calorie_strings)
        .into_iter()
        .map(parse_calorie_line)
        .sum();
    return cals;
}

pub fn day_01(input: String) -> u32 {
    let per_elf_pattern = Regex::new(r"\n[[:space:]]*\n").unwrap();
    let max_cals: u32 = per_elf_pattern
        .split(input.as_str())
        .into_iter()
        .map(compute_per_elf_calories)
        .max()
        .unwrap_or(0);

    return max_cals;
}

pub fn day_01_b(input: String) -> u32 {
    let per_elf_pattern = Regex::new(r"\n[[:space:]]*\n").unwrap();
    let mut per_elf_calories: Vec<u32> = per_elf_pattern
        .split(input.as_str())
        .into_iter()
        .map(compute_per_elf_calories)
        .collect();
    per_elf_calories.sort();
    return per_elf_calories.into_iter().rev().take(3).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_calorie_calc() {
        let input = r#"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "#;

        let result = day_01(input.to_string());
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_top_3_calories() {
        let input = r#"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "#;

        let result = day_01_b(input.to_string());
        assert_eq!(result, 45000);
    }
}
