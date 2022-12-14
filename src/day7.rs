use path_absolutize::Absolutize;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

pub fn main() {
    let mut pwd = PathBuf::new();
    let mut files: HashMap<PathBuf, usize> = Default::default();
    let mut dirs: HashSet<PathBuf> = Default::default();

    include_str!("data/day7.txt").lines().for_each(|line| {
        match line.split(" ").collect::<Vec<_>>()[..] {
            ["$", "cd", dir] => {
                pwd.push(dir);
                pwd = pwd.absolutize().unwrap().into_owned()
            }
            ["$", "ls"] => (),
            ["dir", dir] => {
                dirs.insert({
                    let mut new_p = pwd.clone();
                    new_p.push(dir);
                    new_p
                });
            }
            [size, fname] => {
                let mut new_p = pwd.clone();
                new_p.push(fname);
                files.insert(new_p, size.parse().unwrap());
            }
            _ => unimplemented!(),
        }
    });
    let mut tot_size = 0;
    for dir in dirs {
        let size: usize = files
            .iter()
            .filter_map(|(f, s)| if f.starts_with(&dir) { Some(s) } else { None })
            .sum();
        if size <= 100000 {
            tot_size += size;
        }
    }
    dbg!(tot_size);
}
