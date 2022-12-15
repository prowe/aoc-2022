use std::{collections::VecDeque, fmt::Debug};

pub fn compute_input_inspection_product(rounds: u32, stress_reduction_factor: u32) -> u64 {
    let mut monkeys = vec![
        Monkey {
            items: VecDeque::from([71, 86]),
            inspection_count: 0,
            calc_new: |old| old * 13,
            get_next_monkey: |val| if val % 19 == 0 { 6 } else { 7 },
        },
        Monkey {
            items: VecDeque::from([66, 50, 90, 53, 88, 85]),
            inspection_count: 0,
            calc_new: |old| old + 3,
            get_next_monkey: |val| if val % 2 == 0 { 5 } else { 4 },
        },
        Monkey {
            items: VecDeque::from([97, 54, 89, 62, 84, 80, 63]),
            inspection_count: 0,
            calc_new: |old| old + 6,
            get_next_monkey: |val| if val % 13 == 0 { 4 } else { 1 },
        },
        Monkey {
            items: VecDeque::from([82, 97, 56, 92]),
            inspection_count: 0,
            calc_new: |old| old + 2,
            get_next_monkey: |val| if val % 5 == 0 { 6 } else { 0 },
        },
        Monkey {
            items: VecDeque::from([50, 99, 67, 61, 86]),
            inspection_count: 0,
            calc_new: |old| old * old,
            get_next_monkey: |val| if val % 7 == 0 { 5 } else { 3 },
        },
        Monkey {
            items: VecDeque::from([61, 66, 72, 55, 64, 53, 72, 63]),
            inspection_count: 0,
            calc_new: |old| old + 4,
            get_next_monkey: |val| if val % 11 == 0 { 3 } else { 0 },
        },
        Monkey {
            items: VecDeque::from([59, 79, 63]),
            inspection_count: 0,
            calc_new: |old| old * 7,
            get_next_monkey: |val| if val % 17 == 0 { 2 } else { 7 },
        },
        Monkey {
            items: VecDeque::from([55]),
            inspection_count: 0,
            calc_new: |old| old + 7,
            get_next_monkey: |val| if val % 3 == 0 { 2 } else { 1 },
        },
    ];

    let mod_product = vec![19, 2, 13, 5, 7, 11, 17, 3].iter().product();
    return compute_inspection_product(&mut monkeys, rounds, stress_reduction_factor as u64, mod_product);
}

fn compute_inspection_product(
    monkeys: &mut Vec<Monkey>,
    rounds: u32,
    stress_reduction_factor: u64,
    mod_product: u64,
) -> u64 {
    println!("Mod Product: {:?} stress_reduction_factor: {:?}", mod_product, stress_reduction_factor);

    for i in 0..rounds {
        println!("Round {}", i);
        run_round_for_monkeys(monkeys, stress_reduction_factor, mod_product);
    }

    let mut inspection_counts: Vec<u64> = monkeys.into_iter().map(|m| m.inspection_count).collect();
    inspection_counts.sort();
    inspection_counts.reverse();
    println!("Inspection counts: {:?}", inspection_counts);
    let product = inspection_counts[..2].iter().product();
    return product;
}

fn run_round_for_monkeys(
    monkeys: &mut Vec<Monkey>,
    stress_reduction_factor: u64,
    mod_product: u64,
) {
    for i in 0..monkeys.len() {
        println!("\t {:?}", monkeys[i]);
        while let Some((next_monkey, value)) =
            monkeys[i].process_next_item(stress_reduction_factor, mod_product)
        {
            monkeys[next_monkey].items.push_back(value);
        }
    }
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    inspection_count: u64,
    calc_new: fn(u64) -> u64,
    get_next_monkey: fn(u64) -> usize,
}

impl Monkey {
    fn process_next_item(
        &mut self,
        stress_reduction_factor: u64,
        mod_product: u64,
    ) -> Option<(usize, u64)> {
        let old = self.items.pop_front()?;
        let new = (self.calc_new)(old);
        let bored_val = (new / stress_reduction_factor) % mod_product; // stress_reduction_factor as u32;
        let next_monkey = (self.get_next_monkey)(bored_val);
        // println!("{:?} {:?} {:?} {:?}", old, new, bored_val, next_monkey);
        self.inspection_count += 1;
        return Some((next_monkey, bored_val));
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "Monkey: ({:#?}) {:?}", self.inspection_count, self.items);
    }
}

#[cfg(test)]
mod tests {
    use std::{vec, u64::MAX};

    use super::*;
    use test_case::test_case;

    fn build_sample_monkeys() -> Vec<Monkey> {
        return vec![
            Monkey {
                items: VecDeque::from([79, 98]),
                inspection_count: 0,
                calc_new: |old| old * 19,
                get_next_monkey: |val| if val % 23 == 0 { 2 } else { 3 },
            },
            Monkey {
                items: VecDeque::from([54, 65, 75, 74]),
                inspection_count: 0,
                calc_new: |old| old + 6,
                get_next_monkey: |val| if val % 19 == 0 { 2 } else { 0 },
            },
            Monkey {
                items: VecDeque::from([79, 60, 97]),
                inspection_count: 0,
                calc_new: |old| old * old,
                get_next_monkey: |val| if val % 13 == 0 { 1 } else { 3 },
            },
            Monkey {
                items: VecDeque::from([74]),
                inspection_count: 0,
                calc_new: |old| old + 3,
                get_next_monkey: |val| if val % 17 == 0 { 0 } else { 1 },
            },
        ];
    }

    #[test]
    fn test_process_item() {
        let mut monkeys = build_sample_monkeys();

        assert_eq!(monkeys[0].process_next_item(3, MAX), Some((3, 500)));
        assert_eq!(monkeys[0].process_next_item(3, MAX), Some((3, 620)));
        assert_eq!(monkeys[0].process_next_item(3, MAX), None);
        assert_eq!(monkeys[0].inspection_count, 2);
    }

    #[test]
    fn test_monkey_round() {
        let mut monkeys = build_sample_monkeys();
        run_round_for_monkeys(&mut monkeys, 3, MAX);
        assert_eq!(monkeys[0].items, VecDeque::from([20, 23, 27, 26]));
        assert_eq!(
            monkeys[1].items,
            VecDeque::from([2080, 25, 167, 207, 401, 1046])
        );
        assert_eq!(monkeys[2].items, VecDeque::from([]));
        assert_eq!(monkeys[3].items, VecDeque::from([]));

        run_round_for_monkeys(&mut monkeys, 3, MAX);
        assert_eq!(monkeys[0].items, VecDeque::from([695, 10, 71, 135, 350]));
        assert_eq!(monkeys[1].items, VecDeque::from([43, 49, 58, 55, 362]));
        assert_eq!(monkeys[2].items, VecDeque::from([]));
        assert_eq!(monkeys[3].items, VecDeque::from([]));
    }

    #[test]
    fn test_sample_inspection_product() {
        let mut monkeys = build_sample_monkeys();
        let result = compute_inspection_product(&mut monkeys, 20, 3, MAX);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_pt2_product() {
        let mut monkeys = build_sample_monkeys();
        let mod_product = 23 * 19 * 13 * 17;
        let result = compute_inspection_product(&mut monkeys, 10000, 1, mod_product);
        assert_eq!(result, 2713310158);
    }
}
