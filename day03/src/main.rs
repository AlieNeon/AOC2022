use std::fs;

fn main() {
    const FILE_PATH: &str = "input.txt";
    println!("Hi this is the third day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("The sum of the priorities is: {}", &contents.lines().map(|l| get_priori(get_repeat(l))).sum::<u32>());
    println!("The sum of the badges is: {}", get_groups(&contents).iter().map(|g| get_priori(grup_repeat(g))).sum::<u32>());
}

fn get_priori(c: char) -> u32{
    let priori = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    priori.find(c).expect("must be a letre") as u32 + 1
}

fn get_groups(s: &str) -> Vec<Vec<&str>>{
    let mut out = Vec::new();
    let mut lines = s.lines();
    loop {
        let mut grup = Vec::new();
        if let Some(line) = lines.next() {
            grup.push(line);
            grup.push(lines.next().expect("what"));
            grup.push(lines.next().expect("Wharrr"))
        }else{break;}
        out.push(grup);
    }
    out
}

fn grup_repeat(grup: &Vec<&str>) -> char{
    let mut repeated = Vec::new();
    for c in grup[0].chars(){
        if let Some(i) = grup[1].find(c){
            repeated.push(grup[1].chars().nth(i).expect("should have failed before"));
        }
    };
    for c in repeated {
        if let Some(i) = grup[2].find(c){
            return (grup[2].chars().nth(i).expect("should have failed before"));
        }
    };
    return panic!();
}

fn get_repeat(s: &str) -> char{
    let slen = s.len();
    let s1 = &s[0..slen/2];
    let s2 = &s[slen/2..slen];
    for c in s1.chars(){
        if let Some(i) = s2.find(c){
            return s2.chars().nth(i).expect("should have failed before");
        }
    };
    return panic!();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(input.lines().map(|l| get_priori(get_repeat(l))).sum::<u32>(), 157);
    }
    #[test]
    fn it_works2() {
        let input = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;
        assert_eq!(get_groups(input).iter().map(|g| get_priori(grup_repeat(g))).sum::<u32>(), 70);
    }
}