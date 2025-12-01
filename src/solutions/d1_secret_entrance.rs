use super::Problem;
use std::str::Lines;

pub struct D1_SecretEntrance;

impl Problem for D1_SecretEntrance {
    fn first_part(input: &mut Lines) -> String {
        let mut dial_position: i32 = 50;
        let mut password: u32 = 0;

        for line in input {
            let rotation = Self::parse_rotation(line);

            dial_position += rotation;
            dial_position = dial_position.rem_euclid(100);

            if dial_position == 0 {
                password += 1;
            }
        }

        password.to_string()
    }

    fn second_part(input: &mut Lines) -> String {
        let mut dial_position: i32 = 50;
        let mut password: u32 = 0;

        for line in input {
            let rotation = Self::parse_rotation(line);
            password += (rotation.abs() / 100) as u32;

            let rotation = rotation % 100;

            // if we're at 0 and going left, we have to substract 1 to password
            // since we are not crossing it but we counted it earlier
            if dial_position == 0 && rotation < 0 {
                password -= 1;
            }

            dial_position += rotation;

            if !(0..100).contains(&dial_position) {
                let new_position = dial_position.rem_euclid(100);
                if !(dial_position > 99 && new_position == 0) {
                    password += 1;
                }
                dial_position = new_position;
            }

            if dial_position == 0 {
                password += 1;
            }
        }

        password.to_string()
    }
}

impl D1_SecretEntrance {
    fn parse_rotation(input: &str) -> i32 {
        let sign = if input.starts_with('L') { -1 } else { 1 };

        sign * input[1..].parse::<i32>().unwrap()
    }
}
