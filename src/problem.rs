pub trait Problem {
    fn solve(input: String) -> (String, String) {
        let input: Vec<&str> = Self::format_input(&input);

        (Self::first_part(&input), Self::second_part(&input))
    }

    fn first_part(input: &Vec<&str>) -> String {
        let _ = input;
        String::default()
    }

    fn second_part(input: &Vec<&str>) -> String {
        let _ = input;
        String::default()
    }

    fn format_input(src: &str) -> Vec<&str> {
        src.lines().collect()
    }
}
