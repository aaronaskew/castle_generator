use bracket_lib::prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.print(1, 1, "Hello woyld!")
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
    .with_title("Hello Minimal Bracket World")
    .build()?;

    let gs: State = State {};
    
    main_loop(context, gs)
}
