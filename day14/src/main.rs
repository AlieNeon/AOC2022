use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the fourteenth day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("The ammount of grains of sand with void are: {}", throw_sand_untill_timeout(parse(&contents), (500, 0)).iter().filter(|h| *h.1).count());
	println!("The ammount of grains of sand with a floor are: {}", throw_sand_untill_timeout(add_floor(parse(&contents)), (500, 0)).iter().filter(|h| *h.1).count());
}

use std::cmp::{max,min};
fn parse(s: &str) -> HashMap<(usize, usize), bool> {
    let mut cave = HashMap::new();
    for line in s.lines(){
        let coords: Vec<_> = line.split(" -> ").map(|t| (usize::from_str(t.split(",").next().expect("must exist")).expect("NaN"),usize::from_str(t.split(",").skip(1).next().expect("must exist")).expect("NaN"))).collect();
        for range in coords.windows(2){
            let (ix, iy) = range[0];
            let (fx, fy) = range[1];
            for i in min(ix, fx)..=max(ix, fx){
                for j in min(iy, fy)..=max(iy, fy){
                    cave.insert((i,j), false);
                }
            }
        }
    }
    cave
}

fn throw_sand(mut cave: HashMap<(usize, usize), bool>, hole: (usize, usize)) -> (HashMap<(usize, usize), bool>, bool){
    let mut timeout = 0;
    let (mut sx, mut sy) = hole;
    while timeout < 200{
        if !cave.contains_key(&(sx, sy + 1)){
            sy +=1;
        }else if !cave.contains_key(&(sx-1, sy + 1)){
            sy +=1;
            sx -=1;
        }else if !cave.contains_key(&(sx+1, sy + 1)){
            sy +=1;
            sx +=1;
        }else{
            if let Some(_) = cave.insert((sx, sy), true){
				return (cave, true);
			}else{
				return (cave, false);
			}
        }
        timeout +=1;
    }
    (cave, true)
}

fn throw_sand_untill_timeout(mut cave: HashMap<(usize, usize), bool>, hole: (usize, usize)) -> HashMap<(usize, usize), bool>{
    loop {
        let (binding, timeout) = throw_sand(cave, hole);
        cave = binding;
        if timeout {break cave;}
    }
}

fn add_floor(mut cave: HashMap<(usize, usize), bool>) -> HashMap<(usize, usize), bool>{
	let floor = *cave.iter().map(|((_, y), _)| y).max().expect("must have a lowest point")+2;
	for i in 0..=1000{
		cave.insert((i, floor), false);
	}
	cave
}

#[cfg(test)]
mod test {
    const INPUT: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;
    use super::*;
    #[test]
    fn sand1() {
        assert_eq!(throw_sand_untill_timeout(parse(INPUT), (500, 0)).iter().filter(|h| *h.1).count(), 24);
    }
    #[test]
    fn sand2() {
        assert_eq!(throw_sand_untill_timeout(add_floor(parse(INPUT)), (500, 0)).iter().filter(|h| *h.1).count(), 93);
    }
}