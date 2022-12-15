use std::io::{self, Read};

use crate::{
    day_01::{day_01, day_01_b},
    day_02::{rock_paper_scissors, rock_paper_scissors_pt2},
    day_03::{calc_group_badge_totals, calc_total_of_high_priority},
    day_04::{count_containing_pairs, count_overlapping_pairs}, day_05::{calculate_crane_moves, build_stacks, calculate_crate_mover_9001}, day_06::{calculate_first_marker, calculate_start_of_message_index}, day_07::calculate_directory_size_sum, day_08::{count_visible_trees, calc_max_senic_score}, day_09::{count_tail_positions, count_multi_knot_tail_position}, day_10::{calculate_total_signal, parse_steps_into_ascii_art}, day_11::compute_input_inspection_product,
};
use std::env;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    let day = &args[1];

    println!("Running for {}", day);
    let result = match day.as_str() {
        "1" => day_01(buffer).to_string(),
        "1b" => day_01_b(buffer).to_string(),
        "2" => rock_paper_scissors(buffer).to_string(),
        "2b" => rock_paper_scissors_pt2(buffer).to_string(),
        "3" => calc_total_of_high_priority(buffer.as_str()).to_string(),
        "3b" => calc_group_badge_totals(buffer.as_str()).to_string(),
        "4" => count_containing_pairs(buffer.as_str()).to_string(),
        "4b" => count_overlapping_pairs(buffer.as_str()).to_string(),
        "5" => calculate_crane_moves(buffer.as_str(), build_stacks()),
        "5b" => calculate_crate_mover_9001(buffer.as_str(), build_stacks()),
        "6" => calculate_first_marker(buffer.as_str()).to_string(),
        "6b" => calculate_start_of_message_index(buffer.as_str()).to_string(),
        "7" => calculate_directory_size_sum(buffer.as_str()).to_string(),

        "8" => count_visible_trees(buffer.as_str()).to_string(),
        "8b" => calc_max_senic_score(buffer.as_str()).to_string(),

        "9" => count_tail_positions(buffer.as_str()).to_string(),
        "9b" => count_multi_knot_tail_position(buffer.as_str()).to_string(),

        "10" => calculate_total_signal(buffer.as_str()).to_string(),
        "10b" => "\n".to_owned() + &parse_steps_into_ascii_art(buffer.as_str()),

        "11" => compute_input_inspection_product().to_string(),

        _ => "unknown".to_string(),
    };
    println!("Result: {}", result);

    Ok(())
}
