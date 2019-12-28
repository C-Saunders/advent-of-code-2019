use ordered_float::OrderedFloat;
use std::cmp;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn origin() -> Self {
        Point { x: 0, y: 0 }
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Area {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl Area {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        Area {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
    pub fn from_point_list(points: &Vec<&Point>) -> Self {
        let mut min_x = std::i32::MAX;
        let mut min_y = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut max_y = std::i32::MIN;

        for current_point in points.iter() {
            min_x = cmp::min(current_point.x, min_x);
            min_y = cmp::min(current_point.y, min_y);
            max_x = cmp::max(current_point.x, max_x);
            max_y = cmp::max(current_point.y, max_y);
        }

        Area::new(min_x, min_y, max_x, max_y)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub struct Vector {
    pub end_point: Point,
    pub angle_deg: OrderedFloat<f64>,
    pub distance: OrderedFloat<f64>,
}

impl Vector {
    pub fn from_points(start: &Point, end: &Point) -> Self {
        let y_diff = (start.y - end.y) as f64; // start - end b/c y increases downwards
        let x_diff = (end.x - start.x) as f64;
        Vector {
            end_point: end.clone(),
            angle_deg: OrderedFloat(Vector::to_positive_degrees(y_diff.atan2(x_diff))),
            distance: OrderedFloat((y_diff.powf(2.0) + x_diff.powf(2.0)).sqrt()),
        }
    }

    fn to_positive_degrees(raw_angle_radians: f64) -> f64 {
        let positive_radians = if raw_angle_radians < 0.0 {
            raw_angle_radians + 2f64 * std::f64::consts::PI
        } else {
            raw_angle_radians
        };
        positive_radians * (180f64 / std::f64::consts::PI)
    }
}

#[cfg(test)]
mod from_points {
    use super::*;

    #[test]
    fn points_0deg() {
        assert_eq!(
            Vector::from_points(&Point { x: 10, y: 10 }, &Point { x: 11, y: 10 }),
            Vector {
                end_point: Point { x: 11, y: 10 },
                angle_deg: OrderedFloat(0.0),
                distance: OrderedFloat(1.0)
            }
        );
    }

    #[test]
    fn points_180deg() {
        assert_eq!(
            Vector::from_points(&Point { x: 10, y: 10 }, &Point { x: 9, y: 10 }),
            Vector {
                end_point: Point { x: 9, y: 10 },
                angle_deg: OrderedFloat(180.0),
                distance: OrderedFloat(1.0)
            }
        );
    }

    #[test]
    fn points_90deg() {
        assert_eq!(
            Vector::from_points(&Point { x: 10, y: 10 }, &Point { x: 10, y: 9 }),
            Vector {
                end_point: Point { x: 10, y: 9 },
                angle_deg: OrderedFloat(90.0),
                distance: OrderedFloat(1.0)
            }
        );
    }

    #[test]
    fn points_270deg() {
        assert_eq!(
            Vector::from_points(&Point { x: 10, y: 10 }, &Point { x: 10, y: 11 }),
            Vector {
                end_point: Point { x: 10, y: 11 },
                angle_deg: OrderedFloat(270.0),
                distance: OrderedFloat(1.0)
            }
        );
    }
}

#[cfg(test)]
mod to_positive_degrees {
    use super::*;

    #[test]
    fn to_positive_degrees_360() {
        assert_eq!(Vector::to_positive_degrees(0f64.atan2(1f64)), 0.0);
        assert_eq!(Vector::to_positive_degrees(1f64.atan2(1f64)), 45.0);
        assert_eq!(Vector::to_positive_degrees(1f64.atan2(0f64)), 90.0);
        assert_eq!(Vector::to_positive_degrees(1f64.atan2(-1f64)), 135.0);
        assert_eq!(Vector::to_positive_degrees(0f64.atan2(-1f64)), 180.0);
        assert_eq!(Vector::to_positive_degrees(-1f64.atan2(-1f64)), 225.0);
        assert_eq!(Vector::to_positive_degrees(-1f64.atan2(0f64)), 270.0);
        assert_eq!(Vector::to_positive_degrees(-1f64.atan2(1f64)), 315.0);
    }
}
