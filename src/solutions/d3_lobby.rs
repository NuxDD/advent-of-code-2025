use super::Problem;

use std::collections::HashMap;

pub struct D3_Lobby;

impl Problem for D3_Lobby {
    // second_part could be adapted to first_part as well, way more cleaner & performant...
    fn first_part(input: &Vec<&str>) -> String {
        let mut output_joltage: u64 = 0;

        for &bank in input {
            let mut bank_joltage: u64 = 0;

            let bank_size = bank.len();

            for i in 0..bank_size - 1 {
                for j in i + 1..bank_size {
                    let joltage: u64 = format!("{}{}", &bank[i..i + 1], &bank[j..j + 1])
                        .parse::<u64>()
                        .unwrap();

                    if joltage > bank_joltage {
                        bank_joltage = joltage;
                    }
                }
            }

            output_joltage += bank_joltage;
        }

        output_joltage.to_string()
    }

    fn second_part(input: &Vec<&str>) -> String {
        let mut output_joltage: u64 = 0;
        const LENGTH_OUTPUT: usize = 12;

        for &bank in input {
            let mut bank_joltage: u64 = 0;
            let mut current_position = 0;
            let mut current_length = 0;

            while current_length != LENGTH_OUTPUT {
                let remaining_needed = LENGTH_OUTPUT - current_length - 1;
                let window_end = bank.len() - remaining_needed;

                let (pos, value) = Self::get_greater_battery(&bank[current_position..window_end]);

                bank_joltage += value * 10_u64.pow(remaining_needed as u32);
                current_position += pos + 1;
                current_length += 1;
            }

            output_joltage += bank_joltage;
        }

        output_joltage.to_string()
    }
}

impl D3_Lobby {
    fn get_greater_battery(bank: &str) -> (usize, u64) {
        bank.chars()
            .enumerate()
            .map(|(i, c)| (i, c.to_digit(10).unwrap() as u64))
            .max_by(|a, b| match a.1.cmp(&b.1) {
                std::cmp::Ordering::Equal => b.0.cmp(&a.0),
                other => other,
            })
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_bank_98() {
        let input = vec!["987654321111111"];
        let result = D3_Lobby::first_part(&input);
        assert_eq!(result, "98");
        let result = D3_Lobby::second_part(&input);
        assert_eq!(result, "987654321111");
    }

    #[test]
    fn test_single_bank_89() {
        let input = vec!["811111111111119"];
        let result = D3_Lobby::first_part(&input);
        assert_eq!(result, "89");
        let result = D3_Lobby::second_part(&input);
        assert_eq!(result, "811111111119");
    }

    #[test]
    fn test_full_banks() {
        let input = vec![
            "987654321111111",
            "811111111111119",
            "234234234234278",
            "818181911112111",
        ];
        let result = D3_Lobby::first_part(&input);
        assert_eq!(result, "357");
        let result = D3_Lobby::second_part(&input);
        assert_eq!(result, "3121910778619");
    }
}
