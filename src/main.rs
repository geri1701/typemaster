mod lib;
use crate::lib::*;
use console_engine::{pixel, Color, KeyCode};
use rand::prelude::*;
use std::time::{Duration, Instant};

fn main() {
    let mut rng = thread_rng();
    let mut word_vec = wordfile_2_rand_vec(&mut rng);
    let mut active_words = Vec::new();
    let mut engine = console_engine::ConsoleEngine::init_fill_require(90, 30, 60);
    let mut frame_count = 0;
    let mut typed_chars = 0;
    let mut score_past = 0;
    let mut cpm = 0;
    let mut wpm = 0;
    let mut state = false;
    let mut control = (50, 5);
    let mut difficulty = 1;
    let mut now = Instant::now();
    let mut selection = "Easy";
    let mut highscore = read_highscore_file();
    let license = include_str!("LICENSE");
    let mut show_license = false;
    loop {
        if !state {
            engine.wait_frame();
            engine.check_resize();
            engine.clear_screen();
            engine.rect(
                0,
                0,
                engine.get_width() as i32 - 1,
                engine.get_height() as i32 - 1,
                pixel::pxl('#'),
            );
            engine.print_fbg(
                (engine.get_width() / 2) as i32 - 48,
                3,
                &format!(
                    "                     _____                 __  __           _
                    |_   _|   _ _ __   ___|  \\/  | __ _ ___| |_ ___ _ __
                      | || | | | '_ \\ / _ \\ |\\/| |/ _` / __| __/ _ \\ '__|
                      | || |_| | |_) |  __/ |  | | (_| \\__ \\ ||  __/ |
                      |_| \\__, | .__/ \\___|_|  |_|\\__,_|___/\\__\\___|_|
                          |___/|_|                                       "
                ),
                Color::Blue,
                Color::Reset,
            );
            engine.print((engine.get_width() / 2) as i32 - 2, 10, "MENU");
            engine.print(
                (engine.get_width() / 2) as i32 - 12,
                12,
                "Press ENTER to start.",
            );
            engine.print(
                (engine.get_width() / 2) as i32 - 12,
                13,
                &format!("Press TAB to toggle difficulty: {}", selection),
            );
            engine.print(
                (engine.get_width() / 2) as i32 - 12,
                14,
                &format!("Press F12 to read license."),
            );
            engine.print(
                (engine.get_width() / 2) as i32 - 12,
                15,
                &format!("Press ESC to quit."),
            );
            engine.print(
                (engine.get_width() / 2) as i32 - 12,
                17,
                &format!(
                    "Your Highscore: {}\nYour highest Cpm: {}",
                    highscore.0, highscore.1
                ),
            );
            if engine.is_key_pressed(KeyCode::Esc) {
                break;
            }
            if engine.is_key_pressed(KeyCode::Tab) {
                match selection {
                    "Easy" => selection = "Normal",
                    "Normal" => selection = "Hard",
                    _ => selection = "Easy",
                }
            }
            if engine.is_key_pressed(KeyCode::Enter) {
                state = true;
                match selection {
                    "Easy" => {
                        difficulty = 1;
                        control.0 = 50;
                    }
                    "Normal" => {
                        difficulty = 2;
                        control.0 = 40;
                    }
                    _ => {
                        difficulty = 3;
                        control.0 = 30;
                    }
                }
                now = Instant::now();
                active_words.push(new_active(
                    word_vec[0].to_string(),
                    rand_pos(
                        &mut rng,
                        engine.get_width() - (word_vec[0].to_string().len() as u32 + 2),
                    ),
                    1_i32,
                ));
                word_vec.remove(0);
                word_vec.push(active_words[0].0.to_string());
            }
            engine.draw();
            if engine.is_key_pressed(KeyCode::F(12)) {
                show_license = true;
                state = true;
            }
        } else if show_license == true && state == true {
            engine.wait_frame();
            engine.check_resize();
            engine.clear_screen();
            engine.rect(
                0,
                0,
                engine.get_width() as i32 - 1,
                engine.get_height() as i32 - 1,
                pixel::pxl('#'),
            );
            engine.print(2, 2, "Press ESC to go back to Menu!");
            engine.print(2, 4, &format!("{}", license));

            if engine.is_key_pressed(KeyCode::Esc) {
                show_license = false;
                state = false;
            }
            engine.draw();
        } else {
            loop {
                engine.wait_frame();
                engine.check_resize();
                engine.clear_screen();
                engine.rect(
                    0,
                    0,
                    engine.get_width() as i32 - 1,
                    engine.get_height() as i32 - 1,
                    pixel::pxl('#'),
                );
                engine.line(
                    1,
                    (engine.get_height() - 3) as i32,
                    (engine.get_width() - 2) as i32,
                    (engine.get_height() - 3) as i32,
                    pixel::pxl('^'),
                );
                engine.print(
                    2,
                    (engine.get_height() - 2) as i32,
                    &format!(
                        "Score: {} cpm: {} wpm: {} (Highscore: {} Max-cpm: {})",
                        typed_chars, cpm, wpm, highscore.0, highscore.1
                    ),
                );
                if typed_chars > highscore.0 {
                    highscore.0 = typed_chars;
                } else if cpm > highscore.1 {
                    highscore.1 = cpm
                }

                for (i, _) in active_words.iter().enumerate() {
                    engine.print(
                        active_words[i].1,
                        active_words[i].2.get(),
                        &active_words[i].0,
                    );
                    if frame_count == control.0 {
                        active_words[i].2.set(active_words[i].2.get() + 1);
                    }
                }
                if frame_count == control.0 && active_words[0].2.get() % control.1 == 0 {
                    active_words.push(new_active(
                        word_vec[0].to_string(),
                        rand_pos(
                            &mut rng,
                            engine.get_width() - (word_vec[0].to_string().len() as u32 + 2),
                        ),
                        1_i32,
                    ));
                    word_vec.remove(0);
                    word_vec.push(active_words[0].0.to_string());
                }
                engine.set_pxl(
                    active_words[0].1,
                    active_words[0].2.get(),
                    pixel::pxl_fg(active_words[0].0.chars().next().unwrap(), Color::DarkGreen),
                );
                engine.draw();
                if frame_count >= control.0 {
                    frame_count = 0;
                } else {
                    frame_count += 1;
                }
                if engine.is_key_pressed(KeyCode::Char(active_words[0].0.chars().next().unwrap())) {
                    if active_words[0].0.len() == 1 && active_words.len() == 1 {
                        active_words.push(new_active(
                            word_vec[0].to_string(),
                            rand_pos(
                                &mut rng,
                                engine.get_width() - (word_vec[0].to_string().len() as u32 + 2),
                            ),
                            1_i32,
                        ));
                        word_vec.remove(0);
                    }
                    active_words[0].0.remove(0);
                    typed_chars += 1;
                    if typed_chars == 100
                        || typed_chars == 200
                        || typed_chars == 300
                        || typed_chars == 500
                        || typed_chars == 800
                        || typed_chars == 1500
                        || typed_chars == 2000
                        || typed_chars == 3000
                        || typed_chars == 4000
                        || typed_chars == 5000
                    {
                        if let Some(new_control) = modify_difficulty(difficulty, typed_chars) {
                            control = new_control;
                        }
                    }
                }
                if active_words[0].0.is_empty() {
                    active_words.remove(0);
                }
                if engine.is_key_pressed(KeyCode::Esc) {
                    write_highscore_file(highscore);
                    active_words.clear();
                    state = false;
                    typed_chars = 0;
                    score_past = 0;
                    cpm = 0;
                    wpm = 0;
                    break;
                }
                if now.elapsed() >= Duration::from_secs(20) {
                    cpm = (typed_chars - score_past) * 3;
                    wpm = cpm / 5;
                    score_past = typed_chars;
                    now = Instant::now();
                }
                if active_words[0].2.get() == engine.get_height() as i32 - 2 {
                    write_highscore_file(highscore);
                    active_words.clear();
                    state = false;
                    typed_chars = 0;
                    score_past = 0;
                    cpm = 0;
                    wpm = 0;
                    break;
                }
            }
        }
    }
}
