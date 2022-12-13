use std::fs;
use std::str::FromStr;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the twELFth day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    
    let mut mes = parse(&contents);

    println!("The sum of indexes of ordered packets is: {}", mes.chunks(2).map(|l| l[0] < l[1]).enumerate().filter(|(_, b)| *b).map(|(i,_)| i+1).sum::<usize>());
    mes.push(parse_line("[[2]]"));
    mes.push(parse_line("[[6]]"));
    mes.sort();
    println!("The distress decoder key is: {}", mes.iter().enumerate().filter(|(_ ,i)| **i == parse_line("[[2]]") || **i == parse_line("[[6]]")).map(|(n,_)| n+1).fold(1, |x,y| x*y));
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item{
    Num(i32),
    List(Vec<Item>)
}
use std::cmp::Ordering;
impl PartialOrd for Item{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Item::Num(s) => {
                match other {
                    Item::Num(o) => {
                        s.partial_cmp(o)
                    },
                    Item::List(_) => {
                        (Item::List(vec![self.clone()])).partial_cmp(other)
                    },
                }
            },
            Item::List(s) => {
                match other {
                    Item::List(o) => {
                        s.partial_cmp(o)
                    },
                    Item::Num(_) => {
                        self.partial_cmp(&Item::List(vec![other.clone()]))
                    },
                }
            },
        }
    }
}
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Item::Num(s) => {
                match other {
                    Item::Num(o) => {
                        s.cmp(o)
                    },
                    Item::List(_) => {
                        (Item::List(vec![self.clone()])).cmp(other)
                    },
                }
            },
            Item::List(s) => {
                match other {
                    Item::List(o) => {
                        s.cmp(o)
                    },
                    Item::Num(_) => {
                        self.cmp(&Item::List(vec![other.clone()]))
                    },
                }
            },
        }
    }
}

fn parse_line(s: &str) -> Vec<Item>{
    let mut out = Vec::new();
    let mut s: Vec<_> = s[1..s.len()-1].chars().collect();
    let mut construct = String::new();
    let mut inside = 0;
    while s.len() > 0 {
        let c = s[0];
        if c == '['{
            inside +=1;
            construct.push(s.remove(0));
        } else if c == ']' {
            inside -=1;
            construct.push(s.remove(0));
            if inside == 0 {
                out.push(Item::List(parse_line(&construct)));
                construct = String::new();
            }
        } else if c == ','{
            if construct.len() > 0 && inside == 0 {
                out.push(Item::Num(i32::from_str(&construct).expect("NaN")));
                construct = String::new();
            }
            if inside > 0 {
                construct.push(c);
            }
            s.remove(0);
        } else {
            construct.push(s.remove(0));
        }
    }
    if construct.len() > 0 {
        out.push(Item::Num(i32::from_str(&construct).expect("NaN")));
    }
    out
}

fn parse(s: &str) -> Vec<Vec<Item>>{
    s.lines().filter(|l| *l !="").map(parse_line).collect()
}

#[cfg(test)]
mod test {
    const INPUT: &str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]"#;
    use super::*;
    #[test]
    fn lists1() {
        assert_eq!(parse(INPUT).chunks(2).map(|l| l[0] < l[1]).enumerate().filter(|(_, b)| *b).map(|(i,_)| i+1).sum::<usize>(), 13);
    }
    #[test]
    fn lists2() {
        let mut mes = parse(INPUT);
        mes.push(parse_line("[[2]]"));
        mes.push(parse_line("[[6]]"));
        mes.sort();
        assert_eq!(mes.iter().enumerate().filter(|(_ ,i)| **i == parse_line("[[2]]") || **i == parse_line("[[6]]")).map(|(n,_)| n+1).fold(1, |x,y| x*y), 140);
    }
}