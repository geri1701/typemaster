mod models;
use {
    console_engine::{pixel, Color, ConsoleEngine, KeyCode},
    models::{Game, Model, Page, NAME},
    rand::prelude::*,
};

fn main() {
    let mut state = Model::default();
    let mut engine = ConsoleEngine::init_fill_require(90, 30, 60).unwrap();
    while !state.exit() {
        match state.page() {
            Page::Licence => {
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
                engine.print(2, 4, include_str!("../LICENSE"));
                engine.draw();
                if engine.is_key_pressed(KeyCode::Esc) {
                    state.set_page(Page::Welcome);
                };
            }
            Page::Welcome => {
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
                    &figlet_rs::FIGfont::standard()
                        .unwrap()
                        .convert(NAME)
                        .unwrap()
                        .to_string(),
                    Color::Red,
                    Color::Reset,
                );
                engine.print((engine.get_width() / 2) as i32 - 2, 10, "MENU");
                engine.print(
                    (engine.get_width() / 2) as i32 - 17,
                    12,
                    &format!(
                        r#"Press ENTER to start.
Press TAB to toggle difficulty: {}
Press F12 to read license.
Press ESC to quit.
Your Highscore: {}
Your highest Cpm: {},
"#,
                        state.difficulty().to_str(),
                        state.highscore().0,
                        state.highscore().1
                    ),
                );
                if engine.is_key_pressed(KeyCode::Tab) {
                    state.shift_difficulty();
                }
                if engine.is_key_pressed(KeyCode::Enter) {
                    state.set_page(Page::Game);
                }
                if engine.is_key_pressed(KeyCode::F(12)) {
                    state.set_page(Page::Licence);
                }
                state.set_exit(engine.is_key_pressed(KeyCode::Esc));
                engine.draw();
            }
            Page::Game => {
                let mut rng = thread_rng();
                let mut game = Game::new(&mut rng);
                while state.page() == Page::Game {
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
                            game.typed_chars(),
                            game.cpm(),
                            game.wpm(),
                            state.highscore().0,
                            state.highscore().1
                        ),
                    );
                    game.add(&mut rng, engine.get_width() + 2);
                    for (word, x, y) in game.list() {
                        engine.print(*x, *y, word);
                    }
                    let (_, x, y) = game.list()[0].clone();
                    if engine.is_key_pressed(KeyCode::Esc) || y == (engine.get_height() - 3) as i32
                    {
                        state.set_page(Page::Welcome);
                    }
                    let letter = game.letter();
                    engine.set_pxl(x, y, pixel::pxl_fg(letter, Color::DarkGreen));
                    if engine.is_key_pressed(KeyCode::Char(letter)) {
                        game.del();
                        game.set_control(state.difficulty() as u8);
                    }
                    game.check_time();
                    engine.draw();
                    if state.highscore().0 < game.typed_chars() && state.highscore().1 < game.cpm()
                    {
                        state.set_highscore((game.typed_chars(), game.cpm()));
                    };
                }
            }
        }
    }
}
