use ggez::{graphics, Context, ContextBuilder, GameResult, audio, audio::SoundSource, conf, input};
use ggez::event::{self, EventHandler};
use std::{env, path};

struct Tetris{
    main_theme: audio::Source,
}
impl Tetris{
    pub fn new(ctx: &mut Context) -> GameResult<Tetris>{
        let theme = audio::Source::new(ctx, "/theme.ogg")?;
        let game = Tetris{ main_theme:theme };
        Ok(game)
    }
    fn play_detached(&mut self, _ctx: &mut Context) {
        let _ = self.main_theme.play_detached();
    }
}
impl EventHandler for Tetris{
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {

        Ok(())
    }
fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx, graphics::WHITE);
    graphics::present(ctx)
}
fn key_down_event(&mut self,ctx: &mut Context, keycode: input::keyboard::KeyCode, _keymods: input::keyboard::KeyMods, _repeat: bool){
        match keycode{
            input::keyboard::KeyCode::Key1 => self.play_detached(ctx),
            input::keyboard::KeyCode::Escape => event::quit(ctx),
            _ => (),
        }
}
}




fn main() -> GameResult{
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        path
    } else {
        path::PathBuf::from("./res")
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("Rustris","Jasomo")
        .add_resource_path(resource_dir)
        .window_setup(conf::WindowSetup::default().title("Rustris: A rust tetris clone"))
        .window_mode(conf::WindowMode::default().dimensions(400.0, 800.0))
        .build()
        .expect("could not create ggez context!");
    let tetris = &mut Tetris::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, tetris)
}
