use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

struct Model {}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    let t = app.time;

    draw.background().color(Rgb::new(0.1,0.1,0.1));

    draw.to_frame(app, &frame).unwrap();
}