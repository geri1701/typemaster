use {
    rand::Rng,
    std::{
        collections::{hash_map::Entry, HashMap},
        fs,
        time::{Duration, Instant},
    },
};

pub enum Page {
    Welcome = 0,
    Licence,
    Main,
    Quit,
}

impl Page {
    pub fn esc(&self) -> Self {
        match self {
            Self::Welcome => Self::Quit,
            _ => Self::Welcome,
        }
    }
}

pub enum Level {
    Easy = 0,
    Normal,
    Hard,
}

impl Level {
    pub fn to_str(&self) -> &str {
        match self {
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
        }
    }
    pub fn switch(&self) -> Self {
        match self {
            Self::Easy => Self::Normal,
            Self::Normal => Self::Hard,
            Self::Hard => Self::Easy,
        }
    }
    pub fn control(&self, typed_chars: u32) -> (u32, u32) {
        match (&self, typed_chars) {
            (&Self::Easy, 0) => (50, 5),
            (&Self::Easy, 100) => (45, 5),
            (&Self::Easy, 200) | (&Self::Normal, 0) => (40, 5),
            (&Self::Easy, 300) => (40, 4),
            (&Self::Easy, 500) => (35, 4),
            (&Self::Easy, 800) => (30, 3),
            (&Self::Easy, 1500) => (25, 3),
            (&Self::Easy, 2000) | (&Self::Normal, 800) => (20, 3),
            (&Self::Easy, 3000) | (&Self::Hard, 800) => (15, 3),
            (&Self::Easy, 4000) | (&Self::Normal, 3000) => (14, 3),
            (&Self::Easy, 5000) | (&Self::Normal, 4000) | (&Self::Hard, 2000) => (12, 3),
            (&Self::Normal, 100) => (35, 5),
            (&Self::Normal, 200) | (&Self::Hard, 0) => (30, 5),
            (&Self::Normal, 300) => (25, 4),
            (&Self::Normal, 500) | (&Self::Hard, 300) => (20, 4),
            (&Self::Normal, 1500) => (18, 3),
            (&Self::Normal, 2000) => (16, 3),
            (&Self::Normal, 5000) | (&Self::Hard, 3000) => (10, 3),
            (&Self::Hard, 100) => (25, 5),
            (&Self::Hard, 200) => (20, 5),
            (&Self::Hard, 500) => (15, 4),
            (&Self::Hard, 1500) => (14, 3),
            (&Self::Hard, 4000) => (9, 3),
            _ => (8, 3),
        }
    }
}

pub enum Lang {
    En = 0,
    Su,
}

impl Lang {
    pub fn to_str(&self) -> &str {
        match self {
            Self::En => "English",
            Self::Su => "Finnish",
        }
    }
    pub fn wordlist(&self) -> Vec<String> {
        match self {
            Self::En => include_str!("../assets/wordlist_en.txt"),
            Self::Su => include_str!("../assets/wordlist_su.txt"),
        }
        .split_whitespace()
        .map(str::to_string)
        .collect()
    }
    pub fn switch(&self) -> Self {
        match self {
            Self::En => Self::Su,
            Self::Su => Self::En,
        }
    }
}

pub struct Model {
    pub page: Page,
    pub lang: Lang,
    pub level: Level,
    pub words: Vec<String>, //WORDS LIST
    pub highscore: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub list: Vec<(String, u32, u32)>, //CURRENT LIST
    frame_count: u32,                  //FPS
    score: u32,
    pub scope: HashMap<char, u32>,
    pub cpm: u32,
    pub wpm: u32,
    pub typed_chars: u32,
    now: Instant,
    control: (u32, u32),
}

impl Default for Model {
    fn default() -> Self {
        Self {
            page: Page::Welcome,
            lang: Lang::En,
            level: Level::Easy,
            highscore: Self::read_highscore(),
            width: 124,
            height: 32,
            words: Vec::new(), //WORDS LIST
            list: Vec::new(),  //CURRENT LIST
            frame_count: 0,    //FPS
            score: 0,
            scope: HashMap::new(),
            cpm: 0,
            wpm: 0,
            typed_chars: 0,
            now: Instant::now(),
            control: (50, 5),
        }
    }
}

impl Model {
    pub const NAME: &'static str = "TypeMaster";
    const U8: u32 = 255;
    fn file() -> String {
        format!("{}/.config/{}", std::env::var("HOME").unwrap(), Self::NAME)
    }
    pub fn save_highscore(&self) {
        let (typed_chars, cpm) = self.highscore;
        fs::write(
            Self::file(),
            [
                (typed_chars / Self::U8) as u8,
                (typed_chars % Self::U8) as u8,
                (cpm / Self::U8) as u8,
                (cpm % Self::U8) as u8,
            ],
        )
        .unwrap();
    }
    fn read_highscore() -> (u32, u32) {
        if let Ok(value) = fs::read(Self::file()) {
            (
                (value[0] * Self::U8 as u8 + value[1]) as u32,
                (value[2] * Self::U8 as u8 + value[3]) as u32,
            )
        } else {
            (0, 0)
        }
    }
    pub fn step_game(&mut self) {
        if self.list.is_empty()
            || self.list[0].0.is_empty()
            || (self.list[0].2 % self.control.1 == 0 && self.frame_count == self.control.0)
        {
            let word = self
                .words
                .remove(rand::rng().random_range(0..self.words.len()));
            let x = rand::rng().random_range(0..=self.width - word.len() as u32);
            self.list.push((word, x, 1));
        };
        if self.list[0].2 > self.height - 4 {
            self.page = Page::Welcome;
            self.save_highscore();
        }
        for idx in 0..self.list.len() {
            if self.frame_count == self.control.0 {
                self.list[idx].2 += 1;
            }
        }
        if self.now.elapsed() >= Duration::from_secs(20) {
            self.cpm = (self.typed_chars - self.score) * 3;
            self.wpm = self.cpm / 5;
            self.score = self.typed_chars;
            self.now = Instant::now();
        }
        self.frame_count = match self.frame_count < self.control.0 {
            true => self.frame_count + 1,
            false => 0,
        };
    }
    pub fn letter(&mut self, value: char) {
        if value == self.list[0].0.chars().next().unwrap() {
            self.typed_chars += 1;
            if let Entry::Vacant(entry) = self.scope.entry(value) {
                entry.insert(1);
            } else {
                *self.scope.get_mut(&value).unwrap() += 1;
            }
            if [0, 100, 200, 300, 500, 800, 1500, 2000, 3000, 4000, 5000]
                .contains(&self.typed_chars)
            {
                self.control = self.level.control(self.typed_chars);
            };
            if self.list[0].0.len() > 1 {
                self.list[0].0.remove(0);
            } else {
                self.list.remove(0);
            };
            self.highscore = (
                *[self.highscore.0, self.typed_chars].iter().max().unwrap(),
                *[self.highscore.1, self.cpm].iter().max().unwrap(),
            );
        }
    }
}
