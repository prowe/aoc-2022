use std::{collections::{HashSet, VecDeque}, u32::MAX, iter};

pub fn day_12_pt_1(input: &str) -> u32 {
    let map = Map::from_str(input);
    return map.compute_shortest_path_length();
}

pub fn day_12_pt_2(input: &str) -> u32 {
    let map = Map::from_str(input);
    return map.compute_shortest_from_any_low_point();
}

struct Map {
    grid: Vec<Vec<char>>,
    start_point: Pos,
    signal_point: Pos,
}

impl Map {
    fn from_str(text: &str) -> Self {
        let grid: Vec<Vec<char>> = text.trim()
            .lines()
            .map(str::trim)
            .map(|l| l.chars().collect())
            .collect();
        
        let start_point = Self::find_chars_in_grid(&grid, 'S').first().unwrap().clone();
        let signal_point = Self::find_chars_in_grid(&grid, 'E').first().unwrap().clone();
        let translate_char = |c: &char| match c {
            'S' => 'a',
            'E' => 'z',
            _ => c.to_owned(),
        };
        let translated_grid: Vec<Vec<char>> = grid.iter()
            .map(|row| row.iter().map(translate_char).collect())
            .collect();

        return Self {
            grid: translated_grid,
            start_point,
            signal_point,
        };
    }

    fn find_chars_in_grid(grid: &Vec<Vec<char>>, needle_char: char) -> Vec<Pos> {
        let mut found: Vec<Pos> = Vec::new();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == needle_char {
                    found.push(Pos::new(x, y));
                }
            }
        }
        return found;
    }

    fn compute_shortest_path_length(&self) -> u32 {
        // let visited_with_me: HashSet<Pos> = HashSet::from([self.start_point]);
        // return self.compute_shortest_next_path(self.start_point, &visited_with_me);
        return self.breadth_first_shortest_path(self.start_point).unwrap();
    }

    fn compute_shortest_from_any_low_point(&self) -> u32 {
        return Self::find_chars_in_grid(&self.grid, 'a')
            .iter()
            .filter_map(|start| self.breadth_first_shortest_path(*start))
            .min()
            .unwrap();
    }

    fn breadth_first_shortest_path(&self, start_point: Pos) -> Option<u32> {
        let mut visited = vec![vec![false; self.grid[0].len()]; self.grid.len()];
        let mut to_visit: VecDeque<(Pos, u32)> = VecDeque::new();
        to_visit.push_back((start_point, 0));

        while let Some((cur_pos, dist)) = to_visit.pop_front() {
            if cur_pos == self.signal_point {
                return Some(dist);
            }

            let my_height = self.height(&cur_pos);
            let next_points: Vec<Pos> = cur_pos.neighbors()
                .iter()
                .filter(|p| self.in_bounds(p))
                .filter(|p| self.height(&p) <= my_height + 1)
                .filter(|p| !visited[p.y][p.x])
                .cloned()
                .collect();
            for p in next_points {
                visited[p.y][p.x] = true;
                to_visit.push_back((p, dist+1));
            }
        }
        
        return None;
    }

    fn compute_shortest_next_path(&self, cur_pos: Pos, visited: &HashSet<Pos>) -> u32 {
        if cur_pos == self.signal_point {
            return visited.len() as u32;
        }

        let visited_with_me: HashSet<Pos> = HashSet::from_iter(visited.iter().cloned().chain(iter::once(cur_pos))); 
        let my_height = self.height(&cur_pos);
        return cur_pos.neighbors()
            .iter()
            .cloned()
            .filter(|p| self.in_bounds(p))
            .filter(|p| self.height(&p) <= my_height + 1)
            .filter(|p| !visited_with_me.contains(p))
            .map(|p| self.compute_shortest_next_path(p, &visited_with_me))
            .min().unwrap_or(MAX);
    }

    fn in_bounds(&self, point: &Pos) -> bool {
        return point.x < self.grid[0].len()
            && point.y < self.grid.len();
    }

    fn height(&self, pos: &Pos) -> u32 {
        return self.grid[pos.y][pos.x].to_digit(36).unwrap();
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        return Self {
            x,
            y
        }
    }

    fn neighbors(&self) -> Vec<Pos> {
        let mut neighbors = vec![
            Pos::new(self.x, self.y+1),
            Pos::new(self.x+1, self.y),
        ];
        if self.y > 0 {
            neighbors.push(Pos::new(self.x, self.y-1));
        }
        if self.x > 0 {
            neighbors.push(Pos::new(self.x-1, self.y));
        }
        return neighbors;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_map_from_string() {
        let input = r#"
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
        "#;
        let map = Map::from_str(&input);
        assert_eq!(map.grid, vec![
            vec!['a', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
            vec!['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
            vec!['a', 'c', 'c', 's', 'z', 'z', 'x', 'k'],
            vec!['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
            vec!['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i']
        ]);
        assert_eq!(map.start_point, Pos::new(0, 0));
        assert_eq!(map.signal_point, Pos::new(5, 2));
    }

    #[test]
    fn test_part_1_shortest_path_length() {
        let input = r#"
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
        "#;
        let map = Map::from_str(&input);
        let length = map.compute_shortest_path_length();
        assert_eq!(length, 31);
    }
}