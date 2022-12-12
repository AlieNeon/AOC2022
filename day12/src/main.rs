use pathfinding::prelude::bfs;
use std::fs;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the twELFth day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");
    let (ini, map) = parse(&contents);
    println!("The shortest path is: {}", bfs(&ini, |p| get_succs(*p, &map), |p| map[p.1][p.0] == 'E').expect("no path found").len()-1);
    let mut asses = get_all_a(&map);
    asses.push(ini);
    let asses = asses.iter().map(|ini| bfs(ini, |p| get_succs(*p, &map), |p| map[p.1][p.0] == 'E'))
    .filter(|x| x.is_some())
    .map(|x| x.unwrap().len()-1);
    println!("The shortestest path is: {}", asses.min().expect("Must be a lower bound"));

}

fn get_succs((px, py): (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)>{
    let mut succs = Vec::new();
    let (px, py) = (px as i32, py as i32);
    for (x, y) in [(-1, 0), (1, 0), (0, -1), (0, 1)]{
        if py+y < 0 || py+y >= map.len() as i32 || px+x < 0 || px+x >= map[0].len() as i32 {continue;}
        let (px, py, pxx, pyy) = (px as usize, py as usize, (px+x) as usize, (py+y) as usize);
        match map[py][px] {
            'S' => if 'a' as u8+1 >= map[pyy][pxx] as u8 {succs.push((pxx, pyy))},
            'E' => if 'z' as u8+1 >= map[pyy][pxx] as u8 {succs.push((pxx, pyy))},
            c => if map[pyy][pxx] != 'E' && c as u8+1 >= map[pyy][pxx] as u8
            || map[pyy][pxx] == 'S'
            || (map[pyy][pxx] == 'E' && (c=='y' || c=='z'))
            {succs.push((pxx, pyy))},
        };
    }
    succs
}

fn parse(s: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let map: Vec<Vec<_>> = s.lines().map(|l| l.chars().collect()).collect();
    let mut ini = (0, 0);
    'out: for i in 0..map.len(){
        for j in 0..map[i].len(){
            if map[i][j] == 'S' {ini = (j,i); break 'out;}
        }
    };
    (ini, map)
}

fn get_all_a(map: &Vec<Vec<char>>) -> Vec<(usize, usize)>{
    let mut ini = Vec::new();
    for i in 0..map.len(){
        for j in 0..map[i].len(){
            if map[i][j] == 'a' {ini.push((j,i));}
        }
    };
    ini
}

#[cfg(test)]
mod test {
    const INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;
    use super::*;
    #[test]
    fn path1() {
        let (ini, map) = parse(INPUT);
        assert_eq!(bfs(&ini, |p| get_succs(*p, &map), |p| map[p.1][p.0] == 'E').expect("no path found").len()-1, 31);
    }
    #[test]
    fn path2() {
        let (ini, map) = parse(INPUT);
        let mut asses = get_all_a(&map);
        asses.push(ini);
        let asses = asses.iter().map(|ini| bfs(ini, |p| get_succs(*p, &map), |p| map[p.1][p.0] == 'E').expect("no path found").len()-1);
        assert_eq!(asses.min().expect("Must be a lower bound"), 29);
    }
}