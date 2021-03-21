use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let t = app.time;

    draw.background().color(Rgb::new(0.1,0.1,0.1));

    draw.to_frame(app, &frame).unwrap();
}

struct Grid {
    pub cells: Vec<Cell>,
    pub margin: f32,
    pub row_heights: Vec<f32>,
    pub col_widths: Vec<f32>,
    pub palette: Vec<f32>
}

impl Grid {
    // pub fn new() -> Grid {
    //
    // }

    pub fn draw(){

    }
}

struct Cell{
    color: Color,
    orientation: Direction
}


enum Direction {
    Up,
    Left,
    Down,
    Right,
}

fn direction_theta(dir: Direction) -> f32 {
    match dir {
        Direction::Up=> 0.0,
        Direction::Left=> 90.0,
        Direction::Down => 180.0,
        Direction::Right => 270.0
    }
}