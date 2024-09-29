use rand::{prelude::SliceRandom, rngs::ThreadRng, Rng};
use std::{
    env, fs,
    time::{Duration, Instant},
};

pub const NAME: &str = "TypeMaster";
#[derive(PartialEq, Clone)]
pub enum Page {
    Welcome,
    Licence,
    Game,
}

#[derive(Clone)]
pub enum Difficulty {
    Easy = 1,
    Normal,
    Hard,
}
impl Difficulty {
    pub fn to_str(&self) -> &str {
        match self {
            Difficulty::Easy => "Normal",
            Difficulty::Normal => "Hard",
            Difficulty::Hard => "Easy",
        }
    }
}

pub struct Game<'a> {
    words: Vec<&'a str>,           //WORDS LIST
    list: Vec<(String, i32, i32)>, //CURRENT LIST
    seq: usize,                    // Index current list
    frame_count: u32,              //FPS
    score: u32,
    cpm: u32,
    wpm: u32,
    typed_chars: u32,
    now: Instant,
    control: (u32, i32),
}

impl Game<'_> {
    pub fn typed_chars(&self) -> u32 {
        self.typed_chars
    }
    pub fn cpm(&self) -> u32 {
        self.cpm
    }
    pub fn wpm(&self) -> u32 {
        self.wpm
    }
    pub fn list(&self) -> Vec<(String, i32, i32)> {
        self.list.clone()
    }
    pub fn letter(&self) -> (i32, i32, char) {
        let (word, x, y) = &self.list[0];
        (*x, *y, word.chars().next().unwrap())
    }
    pub fn del(&mut self) {
        if self.list[0].0.len() > 1 {
            self.list[0].0.remove(0);
        } else {
            self.list.remove(0);
        };
        self.typed_chars += 1;
    }
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut words = Vec::new();
        for word in include_str!("../assets/wordlist.txt").lines() {
            words.push(word);
        }
        words.shuffle(rng);
        Self {
            words,
            list: Vec::new(),
            seq: 0,
            frame_count: 0,
            score: 0,
            cpm: 0,
            wpm: 0,
            typed_chars: 0,
            now: Instant::now(),
            control: (50, 5),
        }
    }
    fn check_time(&mut self) {
        if self.now.elapsed() >= Duration::from_secs(20) {
            self.cpm = (self.typed_chars - self.score) * 3;
            self.wpm = self.cpm / 5;
            self.score = self.typed_chars;
            self.now = Instant::now();
        }
    }
    pub fn check_boarder(&self, value: u32) -> bool {
        self.list[0].2 as u32 > value - 3
    }
    pub fn shift(&mut self, idx: usize) {
        if self.frame_count == self.control.0 {
            self.list[idx].2 += 1;
        }
    }
    pub fn step(&mut self, rng: &mut ThreadRng, width: u32) {
        self.check_time();
        self.frame_count = match self.frame_count < self.control.0 {
            true => self.frame_count + 1,
            false => 0,
        };
        if self.list.is_empty()
            || self.list[0].0.is_empty()
            || (self.list[0].2 % self.control.1 == 0 && self.frame_count == self.control.0)
        {
            self.seq = match self.seq < self.words.len() {
                true => self.seq.saturating_add(1),
                false => 0,
            };
            let x = rng.gen_range(2..width - 1 - self.words[self.seq].len() as u32) as i32;
            self.list.push((self.words[self.seq].to_string(), x, 1));
        }
    }
    pub fn set_control(&mut self, difficulty: u8) {
        if [0, 100, 200, 300, 500, 800, 1500, 2000, 3000, 4000, 5000].contains(&self.typed_chars) {
            self.control = match (difficulty, self.typed_chars) {
                (1, 0) => (50, 5),
                (1, 100) => (45, 5),
                (2, 0) | (1, 200) => (40, 5),
                (1, 300) => (40, 4),
                (1, 500) => (35, 4),
                (1, 800) => (30, 3),
                (1, 1500) => (25, 3),
                (2, 100) => (35, 5),
                (3, 0) | (2, 200) => (30, 5),
                (2, 300) => (25, 4),
                (1, 2000) | (2, 800) => (20, 3),
                (2, 1500) => (18, 3),
                (2, 2000) => (16, 3),
                (1, 4000) | (2, 3000) => (14, 3),
                (3, 100) => (25, 5),
                (3, 200) => (20, 5),
                (2, 500) | (3, 300) => (20, 4),
                (3, 500) => (15, 4),
                (1, 3000) | (3, 800) => (15, 3),
                (3, 1500) => (14, 3),
                (1, 5000) | (2, 4000) | (3, 2000) => (12, 3),
                (2, 5000) | (3, 3000) => (10, 3),
                (3, 4000) => (9, 3),
                _ => (8, 3),
            }
        }
    }
}

pub struct Model {
    exit: bool,
    page: Page,
    difficulty: Difficulty,
    highscore: (u32, u32),
}

impl Model {
    pub fn highscore(&self) -> (u32, u32) {
        self.highscore
    }
    pub fn exit(&self) -> bool {
        self.exit
    }
    pub fn page(&self) -> Page {
        self.page.clone()
    }
    pub fn difficulty(&self) -> Difficulty {
        self.difficulty.clone()
    }
    pub fn set_page(&mut self, value: Page) {
        self.page = value;
    }
    pub fn set_highscore(&mut self, value: (u32, u32)) {
        if self.highscore.0 < value.0 {
            self.highscore.0 = value.0
        };
        if self.highscore.1 < value.1 {
            self.highscore.1 = value.1
        };
        self.save_highscore();
    }
    pub fn set_exit(&mut self, value: bool) {
        self.exit = value;
    }
    pub fn default() -> Self {
        Self {
            exit: false,
            page: Page::Welcome,
            highscore: read_highscore(),
            difficulty: Difficulty::Easy,
        }
    }
    pub fn save_highscore(&self) {
        const U8: u32 = 255;
        let (typed_chars, cpm) = self.highscore;
        fs::write(
            file(),
            [
                (typed_chars / U8) as u8,
                (typed_chars % U8) as u8,
                (cpm / U8) as u8,
                (cpm % U8) as u8,
            ],
        )
        .unwrap();
    }
    pub fn shift_difficulty(&mut self) {
        self.difficulty = match self.difficulty {
            Difficulty::Easy => Difficulty::Normal,
            Difficulty::Normal => Difficulty::Hard,
            Difficulty::Hard => Difficulty::Easy,
        }
    }
}

fn read_highscore() -> (u32, u32) {
    const U8: u8 = 255;
    if let Ok(value) = fs::read(file()) {
        (
            (value[0] * U8 + value[1]) as u32,
            (value[2] * U8 + value[3]) as u32,
        )
    } else {
        (0, 0)
    }
}

fn file() -> String {
    env::var("HOME").unwrap() + "/.config/" + NAME
}
