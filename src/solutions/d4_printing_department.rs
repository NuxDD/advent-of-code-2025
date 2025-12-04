use super::Problem;

pub struct D4_PrintingDepartment;

impl Problem for D4_PrintingDepartment {
    fn first_part(input: &Vec<&str>) -> String {
        let mut accessible_rolls: u32 = 0;

        let input: Vec<Vec<char>> = input.iter().map(|&line| line.chars().collect()).collect();

        let height = input.len();

        for i in 0..height {
            let width = input[i].len();
            for j in 0..width {
                if input[i][j] == '.' {
                    continue;
                }

                let mut adjacent_rolls_count: u32 = 0;

                for x in i.saturating_sub(1)..=i + 1 {
                    for y in j.saturating_sub(1)..=j + 1 {
                        if !(x == i && y == j)
                            && Self::is_valid_index(x, y, height, width)
                            && input[x][y] == '@'
                        {
                            adjacent_rolls_count += 1;
                        }
                    }
                }

                if adjacent_rolls_count < 4 {
                    accessible_rolls += 1;
                }
            }
        }

        accessible_rolls.to_string()
    }

    fn second_part(input: &Vec<&str>) -> String {
        let mut removed_rolls: u32 = 0;
        let mut need_to_remove = true;

        let mut input: Vec<Vec<char>> = input.iter().map(|&line| line.chars().collect()).collect();

        let height = input.len();

        while need_to_remove {
            need_to_remove = false;
            for i in 0..height {
                let width = input[i].len();
                for j in 0..width {
                    if input[i][j] == '.' {
                        continue;
                    }

                    let mut adjacent_rolls_count: u32 = 0;

                    for x in i.saturating_sub(1)..=i + 1 {
                        for y in j.saturating_sub(1)..=j + 1 {
                            if !(x == i && y == j)
                                && Self::is_valid_index(x, y, height, width)
                                && (input[x][y] == '@' || input[x][y] == '+')
                            {
                                adjacent_rolls_count += 1;
                            }
                        }
                    }

                    if adjacent_rolls_count < 4 {
                        removed_rolls += 1;
                        input[i][j] = '+';
                        need_to_remove = true;
                    }
                }
            }

            if need_to_remove {
                input
                    .iter_mut()
                    .flatten()
                    .filter(|c| **c == '+')
                    .for_each(|c| *c = '.');
            }
        }

        removed_rolls.to_string()
    }
}

impl D4_PrintingDepartment {
    fn is_valid_index(i: usize, j: usize, height: usize, width: usize) -> bool {
        (0..height).contains(&i) && (0..width).contains(&j)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_rolls() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."
            .lines()
            .collect();
        let result = D4_PrintingDepartment::first_part(&input);
        assert_eq!(result, "13");
        let result = D4_PrintingDepartment::second_part(&input);
        assert_eq!(result, "43");
    }
}
