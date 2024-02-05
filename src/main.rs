use bracket_lib::prelude::*;

#[derive(Debug)]
struct State {
    map: Vec<TileType>,
}

#[derive(Clone, Debug)]
enum TileType {
    Wall,
    Floor,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {

        #[allow(clippy::single_match)]
        match ctx.key {
            Some(VirtualKeyCode::Escape) => {
                // Quit
                ctx.quit();
            }
            _ => {} // Nothing happened
        }

        // Clear the screen
        ctx.cls();

        // Iterate the map array, incrementing coordinates as we go.
        let mut y = 0;
        let mut x = 0;
        for tile in self.map.iter() {
            // Render a tile depending upon the tile type
            match tile {
                TileType::Floor => {
                    ctx.print_color(
                        x,
                        y,
                        RGB::from_f32(0.5, 0.5, 0.5),
                        RGB::from_f32(0., 0., 0.),
                        ".",
                    );
                }
                TileType::Wall => {
                    ctx.print_color(
                        x,
                        y,
                        RGB::from_f32(0.0, 1.0, 0.0),
                        RGB::from_f32(0., 0., 0.),
                        "#",
                    );
                }
            }

            // Move the coordinates
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }
    }
}

impl State {
    fn new() -> Self {
        let mut map = vec![TileType::Floor; 80 * 50];

        for x in 0..80 {
            map[xy_idx(x, 0)] = TileType::Wall;
            map[xy_idx(x, 49)] = TileType::Wall;
        }
        for y in 0..50 {
            map[xy_idx(0, y)] = TileType::Wall;
            map[xy_idx(79, y)] = TileType::Wall;
        }

        Self { map }
    }
}


pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}


pub fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % 80, idx as i32 / 80)
}

fn main() -> BError {
    let mut context = BTermBuilder::simple80x50()
        .with_fullscreen(true)
        .with_title("Castle Generator")
        .build()?;

    context.with_mouse_visibility(true);
    context.with_post_scanlines(true);

    let gs: State = State::new();

    dbg!(&gs);

    main_loop(context, gs)
}
