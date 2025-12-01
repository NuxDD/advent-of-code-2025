use std::str::Lines;

pub trait Problem {
    fn solve(input: Lines) -> (String, String) {
        (
            Self::first_part(&mut input.clone()),
            Self::second_part(&mut input.clone()),
        )
    }

    fn first_part(input: &mut Lines) -> String {
        let _ = input;
        String::default()
    }

    fn second_part(input: &mut Lines) -> String {
        let _ = input;
        String::default()
    }
}
