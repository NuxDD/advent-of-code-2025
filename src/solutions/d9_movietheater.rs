use super::Problem;

type Position = (u64, u64);

pub struct D9_MovieTheater;

impl Problem for D9_MovieTheater {
    fn first_part(input: &Vec<&str>) -> String {
        let mut largest_area: u64 = 0;

        let input: Vec<Position> = input.iter().map(|&line| Self::to_position(line)).collect();

        for from in &input {
            for to in &input {
                let area = Self::compute_area(from, to);
                if area > largest_area {
                    largest_area = area;
                }
            }
        }

        largest_area.to_string()
    }

    fn second_part(input: &Vec<&str>) -> String {
        let mut largest_area: u64 = 0;

        let input: Vec<Position> = input.iter().map(|&line| Self::to_position(line)).collect();

        largest_area.to_string()
    }
}

impl D9_MovieTheater {
    fn to_position(line: &str) -> Position {
        let coords = line.split_once(',').unwrap();
        (
            coords.0.parse::<u64>().unwrap(),
            coords.1.parse::<u64>().unwrap(),
        )
    }

    fn compute_area(from: &Position, to: &Position) -> u64 {
        (from.0.abs_diff(to.0) + 1) * (from.1.abs_diff(to.1) + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn movie_theater_simple_example() {
        let input = "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3".lines().collect();
        let result = D9_MovieTheater::first_part(&input);
        assert_eq!(result, "50");
        let result = D9_MovieTheater::second_part(&input);
        assert_eq!(result, "24");
    }
}
