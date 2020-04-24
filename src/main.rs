use ggez::{graphics, Context, ContextBuilder, GameResult, audio};
use ggez::event::{self, EventHandler};
struct Tetris{


}
impl Tetris{
    pub fn new(_ctx: &mut Context) -> Tetris{
        Tetris{

        }
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
}




fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("Rustris","Jasomo")
        .build()
        .expect("could not create ggez context!");
    let mut tetris = Tetris::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut tetris){
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}
