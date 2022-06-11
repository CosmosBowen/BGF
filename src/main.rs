use ggez::graphics::Rect;
use ggez::winit::window;
use ggez::{
    event::{self, EventHandler},
    graphics::{self, Color},
    Context, ContextBuilder, GameResult,
};
use glam::Vec2;
use quad_rand;

fn main() -> GameResult {
    let cb = ContextBuilder::new("BGF", "Bowen");
    let (mut ctx, event_loop) = cb.build()?;

    let game_of_life = GameOfLife::new(&mut ctx, 50)?;

    event::run(ctx, event_loop, game_of_life);
}

struct GameOfLife {
    initial_state: Vec<Vec<u8>>,
    column_count: u32,
    row_count: u32,
    size: u32,
    frames: usize,
}

impl GameOfLife {
    pub fn new(ctx: &mut Context, zoom_out: u32) -> GameResult<GameOfLife> {
        let window_size = window::Window::inner_size(ctx.gfx.window());
        let width = window_size.width;
        let height = window_size.height;
        println!("width:{}, height:{}, zoom_out:{}", width, height, zoom_out);

        let column_count = zoom_out;
        let grid_size = width / column_count;
        let row_count = height / grid_size;
        println!(
            "column_count:{}, row_count:{}, grid_size:{}",
            column_count, row_count, grid_size
        );
        let mut initial_state: Vec<Vec<u8>> =
            vec![vec![0; (column_count) as usize]; (row_count) as usize];

        for y_grid in 0..row_count {
            for x_grid in 0..column_count {
                initial_state[y_grid as usize][x_grid as usize] = quad_rand::gen_range(0, 2);
            }
        }
        /* 'it' is moving
        initial_state[0][0] = 1;
        initial_state[0][2] = 1;
        initial_state[1][1] = 1;
        initial_state[1][2] = 1;
        initial_state[2][1] = 1;
         */

        Ok(GameOfLife {
            initial_state,
            column_count,
            row_count,
            size: grid_size,
            frames: 0,
        })
    }

    pub fn change_state(&mut self) {
        let mut next_state = self.initial_state.clone();
        for y_grid in 0..self.row_count {
            for x_grid in 0..self.column_count {
                next_state[y_grid as usize][x_grid as usize] = self.apply_rule(y_grid, x_grid);
            }
        }
        self.initial_state = next_state;
    }

    pub fn apply_rule(&self, y_grid: u32, x_grid: u32) -> u8 {
        let current_state = self.initial_state[y_grid as usize][x_grid as usize];
        let mut live_neighbours = 0;

        let y_start = if y_grid == 0 { 0 } else { y_grid - 1 };
        let y_end = if y_grid == self.row_count - 1 {
            self.row_count - 1
        } else {
            y_grid + 1
        };

        let x_start = if x_grid == 0 { 0 } else { x_grid - 1 };
        let x_end = if x_grid == self.column_count - 1 {
            self.column_count - 1
        } else {
            x_grid + 1
        };

        for y in y_start..y_end + 1 {
            for x in x_start..x_end + 1 {
                live_neighbours += self.initial_state[y as usize][x as usize];
            }
        }
        live_neighbours -= current_state;
        if current_state == 1 {
            if live_neighbours < 2 || live_neighbours > 3 {
                0
            } else {
                1
            }
        } else {
            if live_neighbours == 3 {
                1
            } else {
                0
            }
        }
    }
}

impl EventHandler for GameOfLife {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.change_state();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(
            ctx,
            graphics::CanvasLoadOp::Clear([1.0, 1.0, 1.0, 1.0].into()),
        );

        //draw code
        for y_grid in 0..self.row_count {
            for x_grid in 0..self.column_count {
                if self.initial_state[(y_grid) as usize][(x_grid) as usize] == 1 {
                    let square = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        Rect::new(0.0, 0.0, (self.size) as f32, (self.size) as f32),
                        Color::BLACK,
                    );
                    canvas.draw(
                        &square?,
                        Vec2::new((x_grid * self.size) as f32, (y_grid * self.size) as f32),
                    );
                }
            }
        }

        /* change speed
        self.frames += 1;
        if self.frames % 10 == 0 {
            self.change_state();
        }
        */

        canvas.finish(ctx)
    }
}
