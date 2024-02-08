use bracket_lib::prelude::*;
use rltk::{embedded_resource, GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;

const WIDTH: usize = 80;
const HEIGHT: usize = 50;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

embedded_resource!(TILE_FONT, "../resources/example_tiles.png");

fn main() -> rltk::BError {
    use rltk::RltkBuilder;

    link_resource!(TILE_FONT, "resources/example_tiles.png");

    let context = RltkBuilder::new()
    .with_dimensions(WIDTH, HEIGHT)
        .with_tile_dimensions(16, 16)
        .with_font("example_tiles.png", 16, 16)
        .with_simple_console(WIDTH, HEIGHT, "example_tiles.png")
        .with_title("Roguelike Tutorial")
        .with_fullscreen(true)
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    let (rooms, map) = new_map_rooms_and_corridors();
    gs.ecs.insert(map);
    let (player_x, player_y) = rooms[0].center();

    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: 2,
            fg: RGB::named(rltk::WHITE),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Player {})
        .build();

    rltk::main_loop(context, gs)
}
