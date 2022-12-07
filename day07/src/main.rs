use std::fs;
use std::str::FromStr;
use std::collections::HashMap;

fn main() {
    const FILE_PATH: &str = "input";
    println!("Hi this is the sixth day of AOC2022, first we will read the file {}", FILE_PATH);
    let contents = fs::read_to_string(FILE_PATH)
        .expect("Should have been able to read the file");

}
#[derive(Debug)]
enum FileSys<'a>{
    File(u32),
    Dir(HashMap<&'a str, FileSys<'a>>)
}

fn traverse<'a>(root: &'a mut HashMap<&'a str, FileSys<'a>>, ptr: &'a Vec<&'a str>) -> HashMap<&'a str, FileSys<'a>>{
    let mut out = root;
    for dir in *ptr{
        out = match out.get_mut(dir).expect("u messed up the dirs") {
            FileSys::Dir(x) => x,
            _ => panic!("u messed up the dirs, tis a file")
        }
    };
    *out
}

fn parse(s: &str) -> HashMap<&str, FileSys>{
    let mut root = HashMap::new();
    let mut s = s.lines();
    s.next();
    let mut ptr: Vec<&str> = vec![];
    for line in s {
        let mut line = line.split(" ");
        match line.next().expect("lol") {
            "$" => if line.next().expect("must have a command") == "cd"{
                match line.next().expect("must have a name") {
                    ".." => ptr.pop(),
                    name => {ptr.push(name); None},
                };
            }else{continue;},
            "dir" =>{
                let root = traverse(&mut root, &ptr);
                root.insert(line.next().expect("must have a name"), FileSys::Dir(HashMap::new()));
            },
            size =>{
                let root = traverse(&mut root, &ptr);
                root.insert(line.next().expect("must have a name"), FileSys::File(u32::from_str(size).expect("NaN")));
            },
        }
    }
    root
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
    fn coms1_1() {
        dbg!(parse(INPUT));
    }
}