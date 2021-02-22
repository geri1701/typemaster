use rand::{prelude::SliceRandom, rngs::ThreadRng, Rng};
use std::{
    cell::Cell,
    fs::File,
    io::{self, BufRead, Write},
    path::Path,
};
pub struct ActiveWord(pub String, pub i32, pub Cell<i32>);
pub fn modify_difficulty(d: u32, s: u32) -> Option<(i32, i32)> {
    match (d, s) {
        (1, 100) => Some((45, 5)),
        (1, 200) => Some((40, 5)),
        (1, 300) => Some((40, 4)),
        (1, 500) => Some((35, 4)),
        (1, 800) => Some((30, 3)),
        (1, 1500) => Some((25, 3)),
        (1, 2000) => Some((20, 3)),
        (1, 3000) => Some((15, 3)),
        (1, 4000) => Some((14, 3)),
        (1, 5000) => Some((12, 3)),
        (2, 100) => Some((35, 5)),
        (2, 200) => Some((30, 5)),
        (2, 300) => Some((25, 4)),
        (2, 500) => Some((20, 4)),
        (2, 800) => Some((20, 3)),
        (2, 1500) => Some((18, 3)),
        (2, 2000) => Some((16, 3)),
        (2, 3000) => Some((14, 3)),
        (2, 4000) => Some((12, 3)),
        (2, 5000) => Some((10, 3)),
        (3, 100) => Some((25, 5)),
        (3, 200) => Some((20, 5)),
        (3, 300) => Some((20, 4)),
        (3, 500) => Some((15, 4)),
        (3, 800) => Some((15, 3)),
        (3, 1500) => Some((14, 3)),
        (3, 2000) => Some((12, 3)),
        (3, 3000) => Some((10, 3)),
        (3, 4000) => Some((9, 3)),
        (3, 5000) => Some((8, 3)),
        _ => None,
    }
}

pub fn new_active(a: String, b: i32, c: i32) -> ActiveWord {
    ActiveWord(a, b, Cell::new(c))
}

pub fn rand_pos(rng: &mut ThreadRng, max: u32) -> i32 {
    rng.gen_range(1..max) as i32
}

pub fn wordfile_2_rand_vec(rng: &mut ThreadRng) -> Vec<String> {
    let wordlist = include_str!("wordlist.txt");
    let lines = wordlist.lines();
    let mut word_vec = Vec::new();
    for word in lines {
        word_vec.push(word.to_string());
    }
    word_vec.shuffle(rng);
    word_vec
}

pub fn read_highscore_file() -> (u32, u32) {
    let mut highscore = (0, 0);
    let mut input_vec = Vec::new();
    if Path::new("highscore.bin").exists() {
        if let Ok(lines) = read_lines("highscore.bin") {
            for line in lines {
                if let Ok(word) = line {
                    input_vec.push(word);
                }
            }
        }
        if input_vec.len() == 2 {
            highscore.0 = input_vec[0].parse().unwrap();
            highscore.1 = input_vec[1].parse().unwrap();
        }
    }
    highscore
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn write_highscore_file(h: (u32, u32)) {
    let a = h.0;
    let b = h.1;
    let mut file = match File::create("highscore.bin") {
        Err(_) => panic!("couldn' t create highscore.bin"),
        Ok(file) => file,
    };
    let buffer = format!("{}\n{}", a, b);
    match file.write_all(buffer.as_bytes()) {
        Err(_) => panic!("couldn't write to highscore.bin"),
        Ok(b) => b,
    }
}
