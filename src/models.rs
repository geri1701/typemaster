use {
    rand::{prelude::SliceRandom, rngs::ThreadRng, rng, Rng},
    std::{
        env, fs,
        time::{Duration, Instant},
    },
};

pub const NAME: &str = "TypeMaster";

#[derive(Clone)]
pub enum Difficulty {
    Easy = 0,
    Normal,
    Hard,
}

impl Difficulty {
    pub fn to_str(self) -> &'static str {
        ["Normal", "Hard", "Easy"][self as usize]
    }
}

pub struct Game<'a> {
    rng: ThreadRng,
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
    pub fn new(lang: &Lang) -> Self {
        let mut rng = rng();
        let mut words = Vec::new();
        for word in match lang {
            Lang::En => include_str!("../assets/wordlist_en.txt").split_whitespace(),
            Lang::Su => include_str!("../assets/wordlist_su.txt").split_whitespace(),
        } {
            words.push(word);
        }
        words.shuffle(&mut rng);
        Self {
            rng,
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
    pub fn step(&mut self, width: u32) {
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
            let x = self
                .rng
                .random_range(2..width - 1 - self.words[self.seq].len() as u32)
                as i32;
            self.list.push((self.words[self.seq].to_string(), x, 1));
        }
    }
    pub fn set_control(&mut self, difficulty: u8) {
        if [0, 100, 200, 300, 500, 800, 1500, 2000, 3000, 4000, 5000].contains(&self.typed_chars) {
            self.control = match (difficulty, self.typed_chars) {
                (0, 0) => (50, 5),
                (0, 100) => (45, 5),
                (0, 200) | (1, 0) => (40, 5),
                (0, 300) => (40, 4),
                (0, 500) => (35, 4),
                (0, 800) => (30, 3),
                (0, 1500) => (25, 3),
                (0, 2000) | (1, 800) => (20, 3),
                (0, 3000) | (2, 800) => (15, 3),
                (0, 4000) | (1, 3000) => (14, 3),
                (0, 5000) | (1, 4000) | (2, 2000) => (12, 3),
                (1, 100) => (35, 5),
                (1, 200) | (2, 0) => (30, 5),
                (1, 300) => (25, 4),
                (1, 500) | (2, 300)  => (20, 4),
                (1, 1500) => (18, 3),
                (1, 2000) => (16, 3),
                (1, 5000) | (2, 3000) => (10, 3),
                (2, 100) => (25, 5),
                (2, 200) => (20, 5),
                (2, 500) => (15, 4),
                (2, 1500) => (14, 3),
                (2, 4000) => (9, 3),
                _ => (8, 3),
            }
        }
    }
}

pub enum Page {
    Main = 0,
    Welcome,
    Licence,
}

#[derive(Clone)]
pub enum Lang {
    En = 0,
    Su,
}

impl Lang {
    pub fn to_str(self) -> &'static str {
        ["English", "Finnish"][self as usize]
    }
}

pub struct Model {
    exit: bool,
    page: Page,
    lang: Lang,
    difficulty: Difficulty,
    highscore: (u32, u32),
}

impl Default for Model {
    fn default() -> Self {
        Self {
            exit: false,
            page: Page::Welcome,
            lang: Lang::En,
            difficulty: Difficulty::Easy,
            highscore: read_highscore(),
        }
    }
}

impl Model {
    pub fn highscore(&self) -> (u32, u32) {
        self.highscore
    }
    pub fn exit(&self) -> bool {
        self.exit
    }
    pub fn page(&self) -> &Page {
        &self.page
    }
    pub fn lang(&self) -> &Lang {
        &self.lang
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
    pub fn shift_lang(&mut self) {
        self.lang = match self.lang {
            Lang::En => Lang::Su,
            Lang::Su => Lang::En,
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
