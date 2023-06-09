use std::str::FromStr;
use std::fs;

fn main() {
    const FILE_PATH: &str = "input.txt";
    println!("Hi this is the fist day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("And we get the elfs from the file!");
    let mut elfs = get_elfs(&contents);
    println!("The elf with the biggest calories is carring: {}", get_max_elf(&elfs));
    println!("The elf with the 3 biggest calories are carring: {}", get_max_3_elf(&mut elfs));

}

fn get_elfs(input: &str) -> Vec<u32> {
    let binding = input.to_owned();
    let input: Vec<&str> = binding.lines().collect();
    let mut elfs = Vec::from([0]);
    for line in input{
        if line == "" {elfs.push(0); continue;}
        let ptr = elfs.len()-1;
        elfs[ptr] += u32::from_str(line).expect("all content must be numbers");
    }
    elfs
}

fn get_max_elf(elfs: &Vec<u32>) -> u32{
    *elfs.iter().max().expect("there must be a maximum lol")
}

fn get_max_3_elf(elfs: &mut Vec<u32>) -> u32{
    let mut out: u32 = 0;
    elfs.sort();
    for _ in 0..3 {
        out += elfs.pop().expect("There must be at least 3 elfs");
    }
    out
}

#[cfg(test)]
mod test {
    const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
    use super::*;
    #[test]
    fn calories1() {
        let tst = get_elfs(INPUT);
        assert_eq!(get_max_elf(&tst), 24000);
    }
    #[test]
    fn calories2() {
        let mut tst = get_elfs(INPUT);
        assert_eq!(get_max_3_elf(&mut tst), 45000);
    }
}