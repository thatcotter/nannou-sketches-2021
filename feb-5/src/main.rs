use nannou::prelude::*;
use blend::*;

struct Model {
    shapes: Vec<Shape>,
    shape_index: usize
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(1920,1080).view(view).build().unwrap();

    let square = blend::Shape::square_simple( pt2(0.0,0.0), 300.0, srgba(1.0,0.0,0.0,1.0));
    let circle = blend::Shape::circle(pt2(0.0,0.0), 300.0, srgba(0.0,0.0,1.0,1.0));
    let triangle = blend::Shape::triangle_simple(pt2(0.0,0.0), 300.0, srgba(0.0,1.0,0.0,1.0));

    let num_steps = 100;
    let blend1 = blend::blend(&circle, &square, num_steps);
    let blend2 = blend::blend(&square, &triangle, num_steps);
    let blend3 = blend::blend(&triangle, &circle, num_steps);
    let mut blends:Vec<Shape> = Vec::new();

    blends.extend(blend1);
    blends.extend(blend2);
    blends.extend(blend3);

    // print!("{:?}", &blends);

    Model {
        shapes: blends,
        shape_index: 0
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.shape_index  = (app.time * 90.0).floor() as usize;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    // let t = app.time;
    let boundary = app.window_rect();

    draw.background().color(Rgb::new(0.1,0.1,0.1));

    let mut i = 0;

    // for point in model.shapes[model.shape_index % model.shapes.len()].clone().values {
    //     draw.line().start(pt2(0.0,0.0)).end(point.0).color(WHITE).x(boundary.left() * 1.8 + boundary.right() * 2.0);
    // }

    for _ in &model.shapes {
        draw.polyline()
            .points_colored(model.shapes[(model.shape_index + i as usize) % model.shapes.len()].clone().values)
            .rotate((model.shape_index as f32 + i as f32) * 0.04)
            .x(boundary.left() * 1.8 + boundary.right() * 2.0 * (i as f32 / model.shapes.len() as f32));
        i += 1;
    }

    draw.to_frame(app, &frame).unwrap();
}
