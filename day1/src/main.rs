use std::str::FromStr;
use std::fs;

fn main() {
    const FILE_PATH: &str = "input.txt";
    println!("Hi this is the fist day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    println!("The elf with the biggest calories is: {}", get_max_elf(&contents));
}

fn get_max_elf(input: &str) -> u32{
    let binding = input.to_owned();
    let input: Vec<&str> = binding.lines().collect();
    let mut elfs = Vec::new();
    let mut ptr: usize = 0;
    elfs.push(0);
    for line in input{
        if line == "" {ptr+=1; elfs.push(0); continue;}
        elfs[ptr] += u32::from_str(line).expect("all content must be numbers");
    }
    *elfs.iter().max().expect("there must be a maximum lol")
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
        
        assert_eq!(get_max_elf(input), 24000);
    }
}