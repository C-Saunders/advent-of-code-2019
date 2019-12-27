use ordered_float::OrderedFloat;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn origin() -> Self {
        Point { x: 0, y: 0 }
    }

    pub fn angle_between(start: &Point, end: &Point) -> OrderedFloat<f64> {
        let y_diff = (start.y - end.y) as f64;
        let x_diff = (start.x - end.x) as f64;
        OrderedFloat::from(y_diff.atan2(x_diff) * (180f64 / std::f64::consts::PI))
    }
}
