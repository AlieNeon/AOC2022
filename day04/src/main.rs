use std::fs;
use std::str::FromStr;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the fourth day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("The ammount of subranges are: {}", contents.lines().map(|l| get_ranges(l)).filter(|t| ranges_contain(t.0,t.1)).count());
    println!("The ammount of overlapped ranges are: {}", contents.lines().map(|l| get_ranges(l)).filter(|t| ranges_overlap(t.0,t.1)).count());
}

fn ranges_contain(a: (u32,u32), b: (u32,u32)) -> bool {
    !((a.0 > b.0 && a.1 > b.1)||(a.0 < b.0 && a.1 < b.1))
}

fn ranges_overlap(a: (u32,u32), b: (u32,u32)) -> bool {
    !((a.0 > b.1 && a.1 > b.1)||(a.0 < b.0 && a.1 < b.0))
}

fn get_ranges(s: &str) -> ((u32, u32),(u32, u32)) {
    let mut out = s.split(",");
    (get_range(out.next().expect("There must be something")),get_range(out.next().expect("There must be a second elf")))
}

fn get_range(s: &str) -> (u32, u32){
    let mut out = s.split("-").map(|n| u32::from_str(n).expect("Must be a number"));
    (out.next().expect("There must be something"), out.next().expect("There must be a second number"))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        assert_eq!(input.lines().map(|l| get_ranges(l)).filter(|t| ranges_contain(t.0,t.1)).count(), 2);
    }
    #[test]
    fn it_works2() {
        let input = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;
        assert_eq!(input.lines().map(|l| get_ranges(l)).filter(|t| ranges_overlap(t.0,t.1)).count(), 4);
    }
}