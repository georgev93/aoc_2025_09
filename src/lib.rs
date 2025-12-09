pub mod file_parser;
use crate::file_parser::FileParser;

mod tiles;
use crate::tiles::Floor;

pub fn solve_pt1(input_file: &str) -> u64 {
    let floor = Floor::new(input_file);
    floor.get_largest_rectangle()
}

pub fn solve_pt2(input_file: &str) -> u64 {
    0
}

pub fn solve(input_file: &str) -> (u64, u64) {
    (0, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PT1: u64 = 50;
    const EXAMPLE_PT2: u64 = 0;
    const ACTUAL_PT1: u64 = 4771532800;
    const ACTUAL_PT2: u64 = 0;

    // #[test]
    // fn example() {
    //     let my_file = FileParser::new("data/example.txt");
    //     let (part_1, part_2) = solve(my_file.get_str());
    //     assert_eq!(part_1, EXAMPLE_PT1);
    //     assert_eq!(part_2, EXAMPLE_PT2);
    // }

    #[test]
    fn example_pts() {
        let my_file = FileParser::new("data/example.txt");
        assert_eq!(solve_pt1(my_file.get_str()), EXAMPLE_PT1);
        assert_eq!(solve_pt2(my_file.get_str()), EXAMPLE_PT2);
    }

    // #[test]
    // fn actual() {
    //     let my_file = FileParser::new("data/input.txt");
    //     let (part_1, part_2) = solve(my_file.get_str());
    //     assert_eq!(part_1, ACTUAL_PT1);
    //     assert_eq!(part_2, ACTUAL_PT2);
    // }
    //
    #[test]
    fn actual_pts() {
        let my_file = FileParser::new("data/input.txt");
        assert_eq!(solve_pt1(my_file.get_str()), ACTUAL_PT1);
        // assert_eq!(solve_pt2(my_file.get_str()), ACTUAL_PT2);
    }
}
