use std::fs;

fn main() {
    const FILE_PATH: &str = "input.txt";
    println!("Hi this is the second day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("The amount of points you whould get is: {}", get_points(&contents, &get_round));
    println!("The ammount of point you whould get using the real strategy are: {}", get_points(&contents, &get_true_round));

}

fn get_true_round(a: &str, b: &str) -> i32 {
    let b = match b {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => panic!("WTF")
    };
    let a = match a {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("WTF")
    };
    ((b/3+2)+a-1)%3+1+b
}

fn get_round(a: &str, b: &str) -> i32 {
    let tot = match b {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => panic!("WTF")
    };
    let a = match a {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => panic!("WTF")
    };
    return match a - tot{
        1 => 0,
        0 => 3,
        2 => 6,
        -1 => 6,
        -2 => 0,
        _ => panic!("Numers")
    } + tot
}

fn get_points(input: &str, f: &dyn Fn(&str,&str) -> i32) -> i32 {
    input.lines().map(|l| {
        let mut n = l.split_whitespace();
        let a = n.next().expect("input must be correct");
        f(a, n.next().expect("input must be correct"))
    }
    ).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let input = r#"A Y
B X
C Z"#;
        assert_eq!(get_points(input, &get_round), 15);
    }
    #[test]
    fn it_works2() {
        let input = r#"A Y
B X
C Z"#;
        assert_eq!(get_points(input, &get_true_round), 12);
    }
}