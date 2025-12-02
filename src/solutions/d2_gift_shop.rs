use super::Problem;

use std::ops::Range;

pub struct D2_GiftShop;

impl Problem for D2_GiftShop {
    fn first_part(input: &Vec<&str>) -> String {
        let mut invalid_sum: u64 = 0;

        for &range_str in input {
            let range = Self::parse_range(range_str);
            for x in range {
                let x_str = x.to_string();
                if x_str.len() & 1 == 1 {
                    continue;
                }

                let mid = x_str.len() / 2;

                if x_str[0..mid] == x_str[mid..] {
                    invalid_sum += x;
                }
            }
        }

        invalid_sum.to_string()
    }

    fn second_part(input: &Vec<&str>) -> String {
        let mut invalid_sum: u64 = 0;

        for &range_str in input {
            let range = Self::parse_range(range_str);
            'id_loop: for x in range {
                let x_str = x.to_string();
                let mid = x_str.len() / 2;

                'pattern_size_loop: for i in 1..mid + 1 {
                    if x_str.len() % i != 0 {
                        continue;
                    }

                    let pattern = &x_str[0..i];

                    for j in 1..x_str.len() / i {
                        if *pattern != x_str[j * i..(j + 1) * i] {
                            continue 'pattern_size_loop;
                        }
                    }

                    invalid_sum += x;
                    continue 'id_loop;
                }
            }
        }

        invalid_sum.to_string()
    }

    fn format_input(src: &str) -> Vec<&str> {
        src.trim().split(",").collect()
    }
}

impl D2_GiftShop {
    fn parse_range(input: &str) -> Range<u64> {
        let bounds: Vec<u64> = input
            .split('-')
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        bounds[0]..bounds[1]
    }
}
