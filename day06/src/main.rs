use std::fs;
use std::collections::HashSet;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the sixth day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("And the first marker is at: {}", find_marker(&contents, 4));
    println!("And the first message is at: {}", find_marker(&contents, 14));
}

fn find_marker(s: &str, lenght: usize) -> usize {
    let c: Vec<char> = s.chars().collect();
    for i in 0..c.len(){
        let mut hash: HashSet<char> = HashSet::new();
        for j in 0..lenght{
            hash.insert(c[i+j]);
            if hash.len() >lenght-1 {return i+lenght}
        }
    }
    panic!()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn coms1_1() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4), 7);
    }
    #[test]
    fn coms1_2() {
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
    }
    #[test]
    fn coms1_3() {
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
    }
    #[test]
    fn coms1_4() {
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
    }
    #[test]
    fn coms1_5() {
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }
    #[test]
    fn coms2_1() {
        assert_eq!(find_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
    }
    #[test]
    fn coms2_2() {
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
    }
    #[test]
    fn coms2_3() {
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
    }
    #[test]
    fn coms2_4() {
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    }
    #[test]
    fn coms2_5() {
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
    }
}