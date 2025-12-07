use super::Problem;

use std::collections::HashSet;

type Position = (usize, usize);

pub struct D7_Laboratories;

impl Problem for D7_Laboratories {
    fn first_part(input: &Vec<&str>) -> String {
        let mut splits: u64 = 0;

        let mut grid: Vec<Vec<char>> = input
            .iter()
            .map(|&line| line.chars().collect::<Vec<char>>())
            .collect();

        let mut visited_splitters: HashSet<Position> = HashSet::new();

        let starting_pos = Self::get_starting_position(&grid);

        splits += Self::move_beam(&mut grid, starting_pos, &mut visited_splitters);

        splits.to_string()
    }
}

impl D7_Laboratories {
    const SPLITTER: char = '^';
    const BEAM: char = '|';

    fn get_starting_position(grid: &[Vec<char>]) -> Position {
        let x = grid[0].iter().position(|ch| *ch == 'S').unwrap();
        (x, 0)
    }

    fn move_beam(
        grid: &mut [Vec<char>],
        from: Position,
        found_splitters: &mut HashSet<Position>,
    ) -> u64 {
        let (current_x, mut current_y) = (from.0, from.1);
        let mut splitters_found: u64 = 0;
        let grid_height = grid.len();

        while current_y < grid_height && grid[current_y][current_x] != Self::SPLITTER {
            grid[current_y][current_x] = Self::BEAM;
            current_y += 1;
        }

        if current_y != grid_height {
            if found_splitters.insert((current_x, current_y)) {
                splitters_found += 1;
            }

            if grid[current_y][current_x + 1] != Self::BEAM {
                splitters_found +=
                    Self::move_beam(grid, (current_x + 1, current_y), found_splitters);
            }
            if grid[current_y][current_x - 1] != Self::BEAM {
                splitters_found +=
                    Self::move_beam(grid, (current_x - 1, current_y), found_splitters);
            }
        }

        splitters_found
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tachyon_split() {
        let input = ".......S.......\n...............\n.......^.......\n...............\n......^.^......\n...............\n.....^.^.^.....\n...............\n....^.^...^....\n...............\n...^.^...^.^...\n...............\n..^...^.....^..\n...............\n.^.^.^.^.^...^.\n..............."
            .lines()
            .collect();

        let result = D7_Laboratories::first_part(&input);
        assert_eq!(result, "21");
    }
}
