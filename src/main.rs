use bracket_lib::prelude::*;
use rltk::{GameState, Rltk, RGB};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod visibility_system;
use visibility_system::VisibilitySystem;

const WIDTH: usize = 80;
const HEIGHT: usize = 50;

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

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
    gs.ecs.register::<Viewshed>();

    let map: Map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

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
        .with(Viewshed{ visible_tiles : Vec::new(), range: 8, dirty: true })

        .build();

    rltk::main_loop(context, gs)
}
