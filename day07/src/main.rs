use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the seventh day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

    println!("The sum of all dirs less than 100000 is {}", parse(&contents).values().filter(|x| **x<100000).sum::<u32>());
    println!("The smallest directory to erase to make the update is {}", get_smaller_dir_for_update(&contents));
}

fn parse(s: &str) -> HashMap<usize, u32>{
    let mut sizes = HashMap::new();
    let mut ptr: Vec<usize> = vec![];
    let mut linode: usize = 0;
    for line in s.lines() {
        let mut line = line.split(" ");
        match line.next().expect("lol") {
            "$" => if line.next().expect("must have a command") == "cd"{
                match line.next().expect("must have a name") {
                    ".." => {ptr.pop(); ()},
                    _ => {ptr.push(linode); sizes.insert(linode, 0); linode+=1;},
                };
            }else{continue;},
            "dir" =>{
                continue
            },
            size =>{
                ptr.iter().for_each(|d| *sizes.get_mut(d).expect("dir must exist")+= u32::from_str(size).expect("NaN"));
            },
        }
    }
    sizes
}

fn get_smaller_dir_for_update(input: &str) -> u32 {
    let sol = parse(input);
    let target = 30000000 - (70000000 - sol.get(&0).expect("Root must exist"));
    let mut sol: Vec<_> = sol.values().collect();
    sol.sort();
    **sol.iter().filter(|x| ***x > target).next().expect("If there is not enough dir that mean the update is already posible dingus")
}

#[cfg(test)]
mod test {
    const INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;
    use super::*;
    #[test]
    fn update1() {
        assert_eq!(parse(INPUT).values().filter(|x| **x<100000).sum::<u32>(), 95437);
    }
    #[test]
    fn update2() {
        assert_eq!(get_smaller_dir_for_update(INPUT), 24933642);
    }
}