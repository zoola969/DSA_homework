use std::f64::consts::PI;
use svg::node::element::path::Command::{Line, Move};
use svg::node::element::path::Position::Absolute;
use svg::node::element::path::{Command, Data, Parameters};
use svg::node::element::Path;
use svg::Document;

const ANGLE: f64 = PI / 3.;

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }
}

fn get_curve_points(depth: usize, length: f64, alpha: f64, start: Point, end: Point) -> Vec<Point> {
    if depth == 0 {
        return vec![start, end];
    }

    let point_a = Point::new(
        start.x + length * alpha.cos(),
        start.y - length * alpha.sin(),
    );
    let point_b = Point::new(
        point_a.x + length * (alpha + ANGLE).cos(),
        point_a.y - length * (alpha + ANGLE).sin(),
    );
    let point_c = Point::new(
        start.x + 2. * length * alpha.cos(),
        start.y - 2. * length * alpha.sin(),
    );

    let mut v = vec![];
    v.append(&mut get_curve_points(
        depth - 1,
        length / 3.,
        alpha,
        start,
        point_a,
    ));
    v.append(&mut get_curve_points(
        depth - 1,
        length / 3.,
        alpha + ANGLE,
        point_a,
        point_b,
    ));
    v.append(&mut get_curve_points(
        depth - 1,
        length / 3.,
        alpha - ANGLE,
        point_b,
        point_c,
    ));
    v.append(&mut get_curve_points(
        depth - 1,
        length / 3.,
        alpha,
        point_c,
        end,
    ));

    v
}

pub fn draw_koch_curve(depth: usize) {
    let init_length = 10. * 3_f64.powi(depth as i32 + 1);
    let start = Point::new(0., init_length / 3.);
    let end = Point::new(start.x + init_length, init_length / 3.);
    draw(
        get_curve_points(depth, init_length / 3., 0., start, end),
        start,
        init_length,
    );
}

fn draw(points: Vec<Point>, start: Point, init_length: f64) {
    let mut commands = vec![Move(Absolute, Parameters::from((start.x, start.y)))];
    commands.append(
        &mut points
            .iter()
            .skip(1)
            .map(|point| Line(Absolute, Parameters::from((point.x, point.y))))
            .collect::<Vec<Command>>(),
    );
    let data = Data::from(commands);
    let path = Path::new()
        .set("stroke", "black")
        .set("vector-effect", "non-scaling-stroke")
        .set("d", data);
    let document = Document::new()
        .set("viewBox", (0, 0, init_length, 2. * init_length / 3.))
        .add(path);
    svg::save("koch.svg", &document).unwrap();
}
