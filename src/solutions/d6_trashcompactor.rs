use super::Problem;

pub struct D6_TrashCompactor;

impl Problem for D6_TrashCompactor {
    fn first_part(input: &Vec<&str>) -> String {
        let mut grand_total: u64 = 0;

        let mut lines = input.iter().rev();
        let ops = lines
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<&str>>();

        let operands = Self::get_operands(&mut lines);

        for (i, &op) in ops.iter().enumerate() {
            let f: fn(&[u64]) -> u64 = match op {
                "+" => Self::get_sum,
                "*" => Self::get_mult,
                _ => unimplemented!(),
            };

            grand_total += f(&operands[i]);
        }

        grand_total.to_string()
    }

    fn second_part(input: &Vec<&str>) -> String {
        let mut grand_total: u64 = 0;

        let matrix = Self::build_matrix(input);

        let height = matrix.len();
        let width = matrix[0].len();

        // Whatever the init, it is always checked in the loop
        let mut current_op: fn(&[u64]) -> u64 = Self::get_sum;

        let mut operands: Vec<u64> = Vec::new();

        for i in 0..width {
            let mut is_number = false;
            let mut current_number: u64 = 0;

            current_op = match matrix[height - 1][i] {
                '+' => Self::get_sum,
                '*' => Self::get_mult,
                _ => current_op,
            };

            for j in 0..height - 1 {
                if matrix[j][i] != ' ' {
                    is_number = true;
                    let digit = matrix[j][i].to_digit(10).unwrap() as u64;
                    current_number = current_number * 10 + digit;
                }
            }

            if is_number {
                operands.push(current_number);
            } else {
                grand_total += current_op(&operands);
                operands.clear();
            }
        }

        if !operands.is_empty() {
            grand_total += current_op(&operands);
        }

        grand_total.to_string()
    }
}

impl D6_TrashCompactor {
    fn build_matrix(src: &Vec<&str>) -> Vec<Vec<char>> {
        src.iter().map(|&line| line.chars().collect()).collect()
    }

    fn get_operands(it: &mut dyn Iterator<Item = &&str>) -> Vec<Vec<u64>> {
        let mut operands: Vec<Vec<u64>> = Vec::new();

        for &line in it {
            let numbers: Vec<u64> = line
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();

            for (j, &n) in numbers.iter().enumerate() {
                if operands.get(j).is_none() {
                    operands.push(Vec::new());
                }
                operands[j].push(n);
            }
        }

        operands
    }

    fn get_sum(operands: &[u64]) -> u64 {
        operands.iter().sum()
    }

    fn get_mult(operands: &[u64]) -> u64 {
        let mut result: u64 = 1;
        for x in operands {
            result *= x;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_trash_simple_example() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            .lines()
            .collect();
        let result = D6_TrashCompactor::first_part(&input);
        assert_eq!(result, "4277556");
        let result = D6_TrashCompactor::second_part(&input);
        assert_eq!(result, "3263827");
    }
}
