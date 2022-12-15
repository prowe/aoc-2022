use std::{collections::HashSet, fmt::Debug};

pub fn count_tail_positions(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().lines().map(str::trim).collect();
    let mut head_loc = Position { x: 0, y: 0 };
    let mut tail_pos: Vec<Position> = vec![Position { x: 0, y: 0 }];

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let distance: i32 = parts[1].parse().unwrap();
        let delta: (i32, i32) = match parts[0] {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Bad input {:?}", line),
        };
        for _i in 0..distance {
            head_loc.x += delta.0;
            head_loc.y += delta.1;

            let current_tail = tail_pos.last().unwrap();
            if current_tail.is_too_far_away(&head_loc) {
                let diff = current_tail.distance(&head_loc);
                tail_pos.push(match (diff.x, diff.y) {
                    (2, _) => Position {
                        x: head_loc.x - 1,
                        y: head_loc.y,
                    },
                    (-2, _) => Position {
                        x: head_loc.x + 1,
                        y: head_loc.y,
                    },
                    (_, 2) => Position {
                        x: head_loc.x,
                        y: head_loc.y - 1,
                    },
                    (_, -2) => Position {
                        x: head_loc.x,
                        y: head_loc.y + 1,
                    },

                    _ => *current_tail,
                });
            }
        }

        println!("head: {:?} tail: {:?}", head_loc, tail_pos.last().unwrap());
    }

    let uniq_pos: HashSet<&Position> = HashSet::from_iter(tail_pos.iter());
    return uniq_pos.len();
}

pub fn count_multi_knot_tail_position(input: &str) -> usize {
    let lines: Vec<&str> = input.trim().lines().map(str::trim).collect();
    let mut knot_positions: Vec<Position> = init_knot_vec();
    let mut tail_positions: HashSet<Position> = HashSet::new();

    for line in lines {
        let parts: Vec<&str> = line.split(' ').collect();
        let distance: i32 = parts[1].parse().unwrap();
        apply_move_to_knots(&mut knot_positions, parts[0], distance, &mut tail_positions);
    }

    return tail_positions.len();
}

fn apply_move_to_knots(
    knots: &mut Vec<Position>,
    direction: &str,
    distance: i32,
    tail_history: &mut HashSet<Position>,
) {
    let delta: (i32, i32) = match direction {
        "R" => (1, 0),
        "L" => (-1, 0),
        "U" => (0, 1),
        "D" => (0, -1),
        _ => panic!("Bad input {:?}", direction),
    };

    for _i in 0..distance {
        let head = knots[0];
        knots[0] = Position {
            x: head.x + delta.0,
            y: head.y + delta.1,
        };

        for k in 1..knots.len() {
            let my_knot = knots[k];
            let my_head = knots[k - 1];
            if my_knot.is_too_far_away(&my_head) {
                knots[k] = my_knot.compute_new_position(&my_head);
            }
        }
        println!("");
        tail_history.insert(knots.last().unwrap().clone());
    }
}

fn init_knot_vec() -> Vec<Position> {
    let mut knot_positions: Vec<Position> = Vec::with_capacity(10);
    knot_positions.resize(10, Position { x: 0, y: 0 });
    return knot_positions;
}

fn plot_knots(knots: &Vec<Position>) {
    let min_x = knots.iter().map(|p| p.x).min().unwrap();
    let max_x = knots.iter().map(|p| p.x).max().unwrap();
    let min_y = knots.iter().map(|p| p.y).min().unwrap();
    let max_y = knots.iter().map(|p| p.y).max().unwrap();
    let mut plot: Vec<Vec<char>> = vec![
        vec!['.'; (max_x - min_x + 1).try_into().unwrap()];
        (max_y - min_y + 1).try_into().unwrap()
    ];
    for i in 0..knots.len() {
        let k = knots[i];
        let x_index: usize = (k.x - min_x).try_into().unwrap();
        let y_index: usize = (k.y - min_y).try_into().unwrap();
        plot[y_index][x_index] = i.to_string().chars().next().unwrap();
    }

    plot.reverse();
    for row in plot {
        println!("{}", row.into_iter().collect::<String>());
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn is_too_far_away(&self, other: &Position) -> bool {
        return (self.x - other.x).abs() > 1 || (self.y - other.y).abs() > 1;
    }

    fn distance(&self, other: &Position) -> Position {
        return Position {
            x: other.x - self.x,
            y: other.y - self.y,
        };
    }

    fn compute_new_position(&self, head: &Position) -> Position {
        let x_dist = match head.x - self.x {
            d if d < 0 => -1,
            d if d > 0 => 1,
            _ => 0,
        };
        let y_dist = match head.y - self.y {
            d if d < 0 => -1,
            d if d > 0 => 1,
            _ => 0,
        };
        return Position {
            x: self.x + x_dist,
            y: self.y + y_dist,
        };
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_tail_positions() {
        let input = r#"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "#;
        let num = count_tail_positions(input);
        assert_eq!(num, 13);
    }

    #[test]
    fn test_count_multi_knot_tail_position() {
        let input = r#"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "#;
        let num = count_multi_knot_tail_position(input);
        assert_eq!(num, 36);
    }

    #[test]
    fn test_each_step_of_multi_knot_positions() {
        let mut knots = init_knot_vec();
        let mut tail_history: HashSet<Position> = HashSet::new();
        apply_move_to_knots(&mut knots, "R", 5, &mut tail_history);
        assert_eq!(
            knots,
            vec![
                Position { x: 5, y: 0 },
                Position { x: 4, y: 0 },
                Position { x: 3, y: 0 },
                Position { x: 2, y: 0 },
                Position { x: 1, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
                Position { x: 0, y: 0 },
            ]
        );

        apply_move_to_knots(&mut knots, "U", 8, &mut tail_history);
        assert_eq!(
            knots,
            vec![
                Position { x: 5, y: 8 },
                Position { x: 5, y: 7 },
                Position { x: 5, y: 6 },
                Position { x: 5, y: 5 },
                Position { x: 5, y: 4 },
                Position { x: 4, y: 4 },
                Position { x: 3, y: 3 },
                Position { x: 2, y: 2 },
                Position { x: 1, y: 1 },
                Position { x: 0, y: 0 },
            ]
        );

        apply_move_to_knots(&mut knots, "L", 8, &mut tail_history);
        assert_eq!(
            knots,
            vec![
                Position { x: -3, y: 8 },
                Position { x: -2, y: 8 },
                Position { x: -1, y: 8 },
                Position { x: 0, y: 8 },
                Position { x: 1, y: 8 },
                Position { x: 1, y: 7 },
                Position { x: 1, y: 6 },
                Position { x: 1, y: 5 },
                Position { x: 1, y: 4 },
                Position { x: 1, y: 3 },
            ]
        );
    }
}
