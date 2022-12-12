use std::fs;
use std::str::FromStr;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the tenth day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("The sum of the asked for signals is: {}", get_dump(&contents).iter().enumerate().take_while(|(n, _)| n<=&220).map(|(c, rx)| (c+1 ,(c+1) as i32*rx))
    .filter(|(n, _)| (n/20)%2==1).step_by(20).map(|(_, out)| out).sum::<i32>());
    print!("{}", get_pixels(get_dump(&contents)));
}

fn get_pixels(i: Vec<i32>) -> String {
    let mut screen = String::new();
    let mut pixel = 0;
    while pixel < i.len() {
        for r in 0..40{
            if r-1 == i[pixel] || r == i[pixel] || r+1 == i[pixel]{
                screen.push('#')
            }else{
                screen.push('.')
            }
            pixel +=1;
        }
        screen.push_str("\n")
    }
    screen
}

fn get_dump(s: &str) -> Vec<i32> {
    let mut rx = 1;
    let mut history = Vec::new();
    for line in s.lines() {
        let mut line = line.split(" ");
        match line.next().expect("there must be a command") {
            "noop" => history.push(rx),
            "addx" => {
                history.push(rx);
                history.push(rx);
                rx += i32::from_str(line.next().expect("There must be an entry")).expect("NaN");
            }
            _ => unreachable!()
        }
    }
    history
}

#[cfg(test)]
mod test {
    const INPUT: &str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop"#;
    use super::*;
    #[test]
    fn crt1() {
        assert_eq!(get_dump(INPUT).iter().enumerate().take_while(|(n, _)| n<=&220).map(|(c, rx)| (c+1 ,(c+1) as i32*rx))
        .filter(|(n, _)| (n/20)%2==1).step_by(20).map(|(_, out)| out).sum::<i32>(), 13140);
    }
    #[test]
    fn crt2() {
        assert_eq!(get_pixels(get_dump(INPUT)), r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#);
    }
}