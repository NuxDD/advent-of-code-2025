use super::Problem;

pub struct D5_Cafeteria;

impl Problem for D5_Cafeteria {
    fn first_part(input: &Vec<&str>) -> String {
        let mut fresh_ingredients: u32 = 0;

        let (ranges, ids) = Self::get_ranges_and_ids(input);

        let mut ranges = ranges.iter();
        let &(mut min_range, mut max_range) = ranges.next().unwrap();

        for id in ids {
            loop {
                if id < min_range {
                    break;
                }

                if (min_range..=max_range).contains(&id) {
                    fresh_ingredients += 1;
                    break;
                }

                if id > max_range {
                    match ranges.next() {
                        Some(&(a, b)) => {
                            (min_range, max_range) = (a, b);
                            continue;
                        }
                        None => break,
                    }
                }
            }
        }

        fresh_ingredients.to_string()
    }

    fn second_part(input: &Vec<&str>) -> String {
        let mut total_fresh_ingredients: u64 = 0;
        let (ranges, _) = Self::get_ranges_and_ids(input);
        let mut ranges = ranges.iter().peekable();

        while let Some(&(min, mut max)) = ranges.next() {
            while let Some(&&(next_min, next_max)) = ranges.peek()
                && next_min <= max
            {
                max = max.max(next_max);
                ranges.next();
            }

            // Must add 1 to include the max value of the range
            total_fresh_ingredients += max - min + 1;
        }

        total_fresh_ingredients.to_string()
    }
}

impl D5_Cafeteria {
    fn get_ranges_and_ids(src: &[&str]) -> (Vec<(u64, u64)>, Vec<u64>) {
        let parts: Vec<&[&str]> = src.split(|&line| line.is_empty()).collect();
        let (mut ranges, mut ids) = (
            parts[0]
                .iter()
                .map(|&s| {
                    let (a, b) = s.split_once('-').unwrap();
                    (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap())
                })
                .collect::<Vec<(u64, u64)>>(),
            parts[1]
                .iter()
                .map(|&x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>(),
        );

        ranges.sort_by_key(|a| a.0);
        ids.sort();

        (ranges, ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_cafeteria() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32"
            .lines()
            .collect();
        let result = D5_Cafeteria::first_part(&input);
        assert_eq!(result, "3");
        let result = D5_Cafeteria::second_part(&input);
        assert_eq!(result, "14");
    }
}
