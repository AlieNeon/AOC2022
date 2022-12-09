use std::fs;
use std::iter;
use std::str::FromStr;
use std::collections::HashSet;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the nineth day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("The ammount of visited positions are: {}", get_positions(&contents, 1).len());
    println!("The ammount of visited positions are: {}", get_positions(&contents, 9).len());
}

fn get_positions(input: &str, num_knots: usize) -> HashSet<(i32, i32)> {
    let moves: Vec<_> = input.lines().map(|l| {
        let l: Vec<_> = l.split(" ").collect();
        let dir = match &l[0] {
            &"U" => (0, 1),
            &"D" => (0, -1),
            &"L" => (-1, 0),
            &"R" => (1, 0),
            &_ => unreachable!()
        };
        iter::repeat(dir).take(usize::from_str(l[1]).expect("NaN"))
    }).flatten().collect();
    let mut head = (0,0);
    let mut knots = vec![(0,0); num_knots];
    let mut visited = HashSet::new();

    for m in moves {
        head = (head.0+m.0, head.1+m.1);
        knots[0] = move_tail(head, knots[0]);
        for i in 1..num_knots {
            knots[i] = move_tail(knots[i-1], knots[i]);
        }
        visited.insert(knots[num_knots-1]);
    }

    visited
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    if dx.abs() <= 1 && dy.abs() <= 1 {return tail;}

    match (dx, dy) {
        (2,2) => (tail.0+1, tail.1+1),
        (-2,2) => (tail.0-1, tail.1+1),
        (2,-2) => (tail.0+1, tail.1-1),
        (-2,-2) => (tail.0-1, tail.1-1),
        (2,y) => (tail.0+1, tail.1+y),
        (-2,y) => (tail.0-1, tail.1+y),
        (x,2) => (tail.0+x, tail.1+1),
        (x,-2) => (tail.0+x, tail.1-1),
        _ => unreachable!()
    }

}

#[cfg(test)]
mod test {
    const INPUT: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;
    const INPUT2: &str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
    use super::*;
    #[test]
    fn rope1() {
        assert_eq!(get_positions(INPUT, 1).len(), 13);
    }
    #[test]
    fn rope2() {
        assert_eq!(get_positions(INPUT2, 9).len(), 36);
    }
}