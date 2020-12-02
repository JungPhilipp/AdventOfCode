pub mod problem0 {
    use crate::util::parse::parse_lines;

    static INPUT_PATH: &str = "src/problem0/input.txt";

    pub fn solve(path_to_input: &str) {
        let input = parse_lines(path_to_input);
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use test_env_log::test;

        #[test]
        fn test_solve() {
            solve(INPUT_PATH);
        }
    }
}
