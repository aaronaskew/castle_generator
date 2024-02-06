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

impl State {
    /// Returns the correct character to render the wall tile depending on
    /// the neighboring wall tiles.
    fn get_wall_char(&self, x: u32, y: u32) -> char {
        // 2555	╣

        // 2563	║

        // 2551	╗

        // 2557	╝

        // 255F	╚

        // 255A	╔

        // 2554	╩

        // 2569	╦

        // 2566	╠

        // 2560	═

        // 2550	╬

        // ◘

        #[derive(Debug)]
        struct NeighborWalls {
            top: bool,
            bottom: bool,
            left: bool,
            right: bool,
        }

        let mut neighbors = NeighborWalls {
            top: false,
            bottom: false,
            left: false,
            right: false,
        };

        // top
        if y == 0 {
            neighbors.top = false;
        } else if self.map[xy_idx(x as i32, y as i32 - 1)] == TileType::Wall {
            neighbors.top = true;
        }

        // bottom
        if y == HEIGHT as u32 - 1 {
            neighbors.bottom = false;
        } else if self.map[xy_idx(x as i32, y as i32 + 1)] == TileType::Wall {
            neighbors.bottom = true;
        }

        // left
        if x == 0 {
            neighbors.left = false;
        } else if self.map[xy_idx(x as i32 - 1, y as i32)] == TileType::Wall {
            neighbors.left = true;
        }

        // right
        if x == WIDTH as u32 - 1 {
            neighbors.right = false;
        } else if self.map[xy_idx(x as i32 + 1, y as i32)] == TileType::Wall {
            neighbors.right = true;
        }

        match (
            neighbors.top,
            neighbors.bottom,
            neighbors.left,
            neighbors.right,
        ) {
            (true, true, true, true) => '╬',
            (true, true, true, false) => '╣',
            (true, true, false, true) => '╠',
            (true, true, false, false) => '║',
            (true, false, true, true) => '╩',
            (true, false, true, false) => '╝',
            (true, false, false, true) => '╚',
            (true, false, false, false) => '║',
            (false, true, true, true) => '╦',
            (false, true, true, false) => '╗',
            (false, true, false, true) => '╔',
            (false, true, false, false) => '║',
            (false, false, true, true) => '═',
            (false, false, true, false) => '═',
            (false, false, false, true) => '═',
            (false, false, false, false) => '◘',
        }
    }
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
                        self.get_wall_char(x, y),
                    );
                }
            }

            // Move the coordinates
            x += 1;
            if x > WIDTH as u32 - 1 {
                x = 0;
                y += 1;
            }
        }
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

        for _ in 0..1000 {
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
