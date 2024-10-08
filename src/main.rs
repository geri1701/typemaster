mod models;
use {
    console_engine::{pixel, Color, ConsoleEngine, KeyCode},
    models::{Game, Model, Page, NAME},
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
                    pixel::pxl_fg('#', Color::DarkYellow),
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
                    pixel::pxl_fg('#', Color::DarkGreen),
                );
                engine.print_fbg(
                    (engine.get_width() / 2) as i32 - 28,
                    3,
                    &figleter::FIGfont::standard()
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
      TAB to toggle difficulty: {}
      F12 to read license.
      ESC to quit.

 Your Highscore: {}
      Highest Cpm: {},
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
                let mut game = Game::new();
                while state.page() == Page::Game {
                    game.step(engine.get_width() + 2);
                    engine.wait_frame();
                    engine.check_resize();
                    engine.clear_screen();
                    engine.rect(
                        0,
                        0,
                        engine.get_width() as i32 - 1,
                        engine.get_height() as i32 - 1,
                        pixel::pxl_fg('#', Color::DarkBlue),
                    );
                    engine.line(
                        1,
                        (engine.get_height() - 3) as i32,
                        (engine.get_width() - 2) as i32,
                        (engine.get_height() - 3) as i32,
                        pixel::pxl_fg('^', Color::DarkRed),
                    );
                    engine.print(
                        2,
                        (engine.get_height() - 2) as i32,
                        &format!(
                            "Score: {}; Chars per minute: {}; Words per minute: {}; (Highscore: {} Max-cpm: {})",
                            game.typed_chars(),
                            game.cpm(),
                            game.wpm(),
                            state.highscore().0,
                            state.highscore().1,
                        ),
                    );
                    for (idx, (word, x, y)) in game.list().iter().enumerate() {
                        game.shift(idx);
                        engine.print(*x, *y, word);
                    }
                    let (x, y, letter) = game.letter();
                    engine.set_pxl(x, y, pixel::pxl_fg(letter, Color::DarkGreen));
                    if engine.is_key_pressed(KeyCode::Esc)
                        || game.check_boarder(engine.get_height())
                    {
                        state.set_page(Page::Welcome);
                    }
                    if engine.is_key_pressed(KeyCode::Char(letter)) {
                        game.del();
                        game.set_control(state.difficulty() as u8);
                        state.set_highscore((game.typed_chars(), game.cpm()));
                    }
                    engine.draw();
                }
            }
        }
    }
}
