use std::fs;
use std::str::FromStr;
use std::collections::BinaryHeap;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the nineth day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("The level of monkey business in this situation is: {}", BinaryHeap::from(play_monke(parse(&contents), 20, Some(3))).iter().take(2).fold(1, |x,y| x*y ));
    println!("The level of monkey business after a lot is: {}", BinaryHeap::from(play_monke(parse(&contents), 10000, None)).iter().take(2).fold(1, |x,y| x*y ));
}
#[derive(Debug)]
enum Operation{
    Sum(u128),
    Mul(u128),
    Pow(u128)
}
#[derive(Debug)]
struct Monke{
    items: Vec<u128>,
    op: Operation,
    test: (u128, usize, usize)
}

fn parse(s: &str) -> Vec<Monke>{
    s.split("\n\n").map(|m|{
        let m: Vec<_> = m.lines().collect();
        let items = m[1].split(": ").skip(1).next().unwrap().split(", ")
            .map(|n| u128::from_str(n).expect("NaN")).collect();
        let op: Vec<_> = m[2].split("old ").skip(1).next().unwrap().split(" ").collect();
        let op = match op[0] {
            "+" => Operation::Sum(u128::from_str(op[1]).expect("NaN")),
            "*" => if op[1] == "old"{
                Operation::Pow(2)
            }else{
                Operation::Mul(u128::from_str(op[1]).expect("NaN"))
            },
            _ => unreachable!()
        };
        let test = (u128::from_str(m[3].split("by ").skip(1).next().unwrap()).expect("NaN"),
        usize::from_str(m[4].split("monkey ").skip(1).next().unwrap()).expect("NaN"),
        usize::from_str(m[5].split("monkey ").skip(1).next().unwrap()).expect("NaN"));

        Monke{items, op, test}
    }).collect()
}

fn play_monke(mut monkes: Vec<Monke>, r: u32, relief: Option<u128>) -> Vec<usize>{
    let mut inspetions = vec![0; monkes.len()];
    let mcm = monkes.iter().map(|m| m.test.0).fold(1, |x,y| x*y);
    for _ in 0..r{
        for i in 0..monkes.len() {
            inspetions[i] += monkes[i].items.len();
            while monkes[i].items.len() > 0{
                let mut item = monkes[i].items.pop().expect("Must have an item");
                item = match monkes[i].op {
                    Operation::Sum(n) => item+n,
                    Operation::Mul(n) => item*n,
                    Operation::Pow(n) => item.pow(n as u32),
                };
                item /= relief.unwrap_or(1);
                item %= mcm;
                let (tes, si, no) = monkes[i].test;
                match item % tes {
                    0 => monkes[si].items.push(item),
                    _ => monkes[no].items.push(item),
                };
            }
        }
    }
    inspetions
}

#[cfg(test)]
mod test {
    const INPUT: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;
    use super::*;
    #[test]
    fn monke1() {
        assert_eq!(BinaryHeap::from(play_monke(parse(INPUT), 20, Some(3))).iter().take(2).fold(1, |x,y| x*y ), 10605);
    }
    #[test]
    fn monke2() {
        assert_eq!(BinaryHeap::from(play_monke(parse(INPUT), 10000, None)).iter().take(2).fold(1, |x,y| x*y), 2713310158);
    }
}