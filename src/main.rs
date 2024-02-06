add_wasm_support!();
use bracket_lib::prelude::*;

const WIDTH: usize = 80;
const HEIGHT: usize = 50;

#[derive(Debug)]
struct State {
    map: Vec<TileType>,
}

#[derive(Clone, Debug, PartialEq)]
enum TileType {
    Wall,
    Floor,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut draw_batch = DrawBatch::new();

        #[allow(clippy::single_match)]
        match ctx.key {
            Some(VirtualKeyCode::Escape) => {
                // Quit
                ctx.quit();
            }
            _ => {} // Nothing happened
        }

        // Clear the screen
        draw_batch.target(0);
        draw_batch.cls();

        // Iterate the map array, incrementing coordinates as we go.
        let mut y = 0;
        let mut x = 0;
        for tile in self.map.iter() {
            let fg = RGB::from_f32(1.0, 1.0, 1.0);
            let bg = RGB::from_f32(0., 0., 0.);

            let glyph = match tile {
                TileType::Wall => 0,
                TileType::Floor => 1,
            };

            draw_batch.set(Point::new(x, y), ColorPair::new(fg, bg), glyph);

            // Move the coordinates
            x += 1;
            if x > WIDTH as u32 - 1 {
                x = 0;
                y += 1;
            }
        }

        draw_batch.submit(0).expect("Batch error");

        render_draw_buffer(ctx).expect("Render error");
    }
}

impl State {
    fn new() -> Self {
        let mut map = vec![TileType::Floor; WIDTH * HEIGHT];

        for x in 0..WIDTH as i32 {
            map[xy_idx(x, 0)] = TileType::Wall;
            map[xy_idx(x, HEIGHT as i32 - 1)] = TileType::Wall;
        }
        for y in 0..HEIGHT as i32 {
            map[xy_idx(0, y)] = TileType::Wall;
            map[xy_idx(WIDTH as i32 - 1, y)] = TileType::Wall;
        }

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..400 {
            let x = rng.roll_dice(1, WIDTH as i32) - 1;
            let y = rng.roll_dice(1, HEIGHT as i32) - 1;
            let idx = xy_idx(x, y);
            map[idx] = TileType::Wall;
        }

        Self { map }
    }
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * WIDTH) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    (idx as i32 % WIDTH as i32, idx as i32 / WIDTH as i32)
}

embedded_resource!(TILE_FONT, "../resources/example_tiles.png");

fn main() -> BError {
    let mut context = BTermBuilder::new()
        .with_dimensions(WIDTH, HEIGHT)
        .with_tile_dimensions(16, 16)
        .with_font("example_tiles.png", 16, 16)
        .with_simple_console(WIDTH, HEIGHT, "example_tiles.png")
        .with_fullscreen(true)
        .with_title("Castle Generator")
        .build()?;

    // context.with_mouse_visibility(true);
    context.with_post_scanlines(true);

    let gs: State = State::new();

    dbg!(&gs);

    main_loop(context, gs)
}
