use rltk::RltkBuilder;
use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

fn main() -> rltk::BError {
    let ctx = RltkBuilder::simple80x50().with_title("Soso game").build()?;
    let mut gs = State { wd: World::new() };
    gs.wd.register::<Position>();
    gs.wd.register::<Renderable>();
    gs.wd.register::<LeftMover>();
    gs.wd.register::<Player>();

    gs.wd
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('M'),
            fg: RGB::named(rltk::BLUE),
            bg: RGB::named(rltk::WHITE),
        })
        .with(Player {})
        .build();

    (0..9).for_each(|x| {
        gs.wd
            .create_entity()
            .with(Position { x: x * 7, y: 20 })
            .with(Renderable {
                glyph: rltk::to_cp437('b'),
                fg: RGB::named(rltk::YELLOW),
                bg: RGB::named(rltk::VIOLET),
            })
            .with(LeftMover {})
            .build();
    });

    rltk::main_loop(ctx, gs)
}

struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

struct State {
    wd: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.wd);
        self.wd.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let positions = self.wd.read_storage::<Position>();
        let renderables = self.wd.read_storage::<Renderable>();

        (&positions, &renderables).join().for_each(|(pos, render)| {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        });
    }
}

#[derive(Component)]
struct LeftMover {}

struct LeftWalker {}
impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        (&lefty, &mut pos).join().for_each(|(_, pos)| {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        })
    }
}

#[derive(Component, Debug)]
struct Player {}

fn move_player(dx: i32, dy: i32, wd: &mut World) {
    let mut poss = wd.write_storage::<Position>();
    let mut players = wd.write_storage::<Player>();

    (&mut players, &mut poss).join().for_each(|(_, pos)| {
        pos.x = min(79, max(0, pos.x + dx));
        pos.y = min(49, max(0, pos.y + dy));
    });
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    ctx.key.map(map_move).flatten().iter().for_each(|(x, y)| {
        move_player(*x, *y, &mut gs.wd);
    });
}

fn map_move(key: VirtualKeyCode) -> Option<(i32, i32)> {
    match key {
        VirtualKeyCode::Left => Some((-1, 0)),
        VirtualKeyCode::Right => Some((1, 0)),
        VirtualKeyCode::Up => Some((0, -1)),
        VirtualKeyCode::Down => Some((0, 1)),
        _ => None,
    }
}
