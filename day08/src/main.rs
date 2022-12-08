use std::fs;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the seventh day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("The ammount of visible trees are {}", visible(parse(&contents)));
    println!("The best score is {}", get_best_score(parse(&contents)));
}

fn visible(forest: Vec<Vec<u32>>) -> u32{
    let mut visible = 0;
    for i in 0..forest.len(){
        for j in 0..forest[i].len(){
            if i == 0 || j == 0 || i == forest.len()-1 || j == forest[i].len()-1 {
                visible +=1;
            } else {
                let tree = forest[i][j];
                if (0..i).map(|n| forest[n][j]).filter(|t| *t>= tree).count() == 0{visible+=1;continue;};
                if (i+1..forest.len()).map(|n| forest[n][j]).filter(|t| *t>= tree).count() == 0{visible+=1;continue;};
                if (0..j).map(|n| forest[i][n]).filter(|t| *t>= tree).count() == 0{visible+=1;continue;};
                if (j+1..forest[i].len()).map(|n| forest[i][n]).filter(|t| *t>= tree).count() == 0{visible+=1;continue;};
            }
        }
    }
    visible
}

fn parse(s: &str) -> Vec<Vec<u32>> {
    s.lines().map(|l| l.chars().map(|c| c.to_digit(10).expect("NaN")).collect()).collect()
}

fn get_best_score(forest: Vec<Vec<u32>>) -> usize{
    let mut score: Vec<usize> = Vec::new();
    for i in 0..forest.len(){
        for j in 0..forest[i].len(){
            if i == 0 || j == 0 || i == forest.len()-1 || j == forest[i].len()-1 {score.push(0);continue;}
            let tree = forest[i][j];
            let mut a = (0..i).rev().map(|n| forest[n][j]).take_while(|t| *t < tree).count();
            let mut b = (i+1..forest.len()).map(|n| forest[n][j]).take_while(|t| *t < tree).count();
            let mut c = (0..j).rev().map(|n| forest[i][n]).take_while(|t| *t < tree).count();
            let mut d = (j+1..forest[i].len()).map(|n| forest[i][n]).take_while(|t| *t < tree).count();
            if a != (0..i).rev().count(){a+=1;}
            if b != (i+1..forest.len()).count(){b+=1;}
            if c != (0..j).rev().count(){c+=1;}
            if d != (j+1..forest[i].len()).count(){d+=1;}
            score.push(a*b*c*d);
        }
    }
    *score.iter().max().expect("there must be a score")
}

#[cfg(test)]
mod test {
    const INPUT: &str = r#"30373
25512
65332
33549
35390"#;
    use super::*;
    #[test]
    fn treehouse1() {
        assert_eq!(visible(parse(INPUT)), 21);
    }
    #[test]
    fn treehouse2() {
        assert_eq!(get_best_score(parse(INPUT)), 8);
    }
}
