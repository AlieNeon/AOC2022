use std::fs;
use std::str::FromStr;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the fifth day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("First we parse this mess");
    let (snacks, moves) = parse(&contents);
    println!("Then we do the moves as told!");
    let snacks = operate9000(snacks, moves);
    println!("Aaaand, the top crates are: {}", peek(&snacks));
    println!("Aw, it was a CrateMover 9001, no problem we make snacks!");
    let (snacks, moves) = parse(&contents);
    println!("Then we do the moves as told, this time the correct way!");
    let snacks = operate9001(snacks, moves);
    println!("Aaaand, the top crates are: {}", peek(&snacks));
}

fn peek(snacks: &Vec<Vec<char>>) -> String{
    let mut top = String::new();
    for s in snacks{
        let ptr = s.len()-1;
        top.push(s[ptr]);
    }
    top
}

fn operate9000(snacks: Vec<Vec<char>>, moves: Vec<(usize, usize, usize)>) -> Vec<Vec<char>>{
    let mut snacks = snacks.to_owned();
    for mov in moves{
        for _ in 0..mov.0{
            let c = snacks[mov.1-1].pop().expect("You must be able to do the isntruction");
            snacks[mov.2-1].push(c);
        }
    }
    snacks
}

fn operate9001(snacks: Vec<Vec<char>>, moves: Vec<(usize, usize, usize)>) -> Vec<Vec<char>>{
    let mut snacks = snacks.to_owned();
    for mov in moves{
        let mut c = Vec::new();
        for _ in 0..mov.0{
            c.push(snacks[mov.1-1].pop().expect("You must be able to do the isntruction"));
        }
        for _ in 0..c.len(){
            snacks[mov.2-1].push(c.pop().expect("Always full here lemao"));
        }
    }
    snacks
}

fn parse(s: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>){
    let mut s = s.split("\n\n");
    let mut snacks = Vec::new();
    let mut moves = Vec::new();
    //Stacks
    let mut stacks = s.next().expect("there must be things in the stacks").lines().rev();
    let n: Vec<char> = stacks.next().expect("REASON").chars().rev().collect();
    let n: usize = n[1].to_digit(10).expect("Must be a number") as usize;
    for _ in 0..n{
        snacks.push(Vec::new())
    }
    for line in stacks{
        let line: Vec<char> = line.chars().collect();
        for i in 0..n{
            let caja = line[i*4+1];
            if caja != ' '{
                snacks[i].push(caja);
            }
        }
    }
    //Moves
    for line in s.next().expect("There must be instructions").lines(){
        let line: Vec<&str> = line.split(" ").collect();
        moves.push((usize::from_str(line[1]).expect("Must be a number"), usize::from_str(line[3]).expect("Must be a number"), usize::from_str(line[5]).expect("Must be a number")));
    }

    (snacks, moves)
}

#[cfg(test)]
mod test {
    const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;
    use super::*;
    #[test]
    fn stacks1() {
        let (snacks, moves) = parse(INPUT);
        let snacks = operate9000(snacks, moves);
        assert_eq!(peek(&snacks), "CMZ");
    }
    #[test]
    fn stacks2() {
        let (snacks, moves) = parse(INPUT);
        let snacks = operate9001(snacks, moves);
        assert_eq!(peek(&snacks), "MCD");
    }
}