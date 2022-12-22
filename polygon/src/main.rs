#[derive(Debug, Copy, Clone, PartialEq)]

pub struct Point {
    // add fields
    x: i32,
    y: i32,
}

impl Point {
    // add methods
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn magnitude(self) -> f64 {
        f64::from(self.x.pow(2) + self.y.pow(2)).sqrt()
    }

    pub fn dist(self, other: Point) -> f64 {
        (self - other).magnitude()
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

pub struct Polygon {
    // add fields
    points: Vec<Point>,
}

impl Polygon {
    // add methods
    fn new() -> Self {
        Self { points: Vec::new() }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point)
    }

    pub fn left_most_point(&self) -> Option<Point> {
        self.points.iter().min_by_key(|p| p.x).copied()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Point> {
        self.points.iter()
    }
}

// pub struct Circle {
//     // add fields
// }

// impl Circle {
//     // add methods
// }

// pub enum Shape {
//     Polygon(Polygon),
//     Circle(Circle),
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    // #[test]
    // fn test_shape_circumferences() {
    //     let mut poly = Polygon::new();
    //     poly.add_point(Point::new(12, 13));
    //     poly.add_point(Point::new(16, 16));
    //     let shapes = vec![
    //         Shape::from(poly),
    //         Shape::from(Circle::new(Point::new(10, 20), 5)),
    //     ];
    //     let circumferences = shapes
    //         .iter()
    //         .map(Shape::circumference)
    //         .map(round_two_digits)
    //         .collect::<Vec<_>>();
    //     assert_eq!(circumferences, vec![10.0, 31.42]);
    // }
}

#[allow(dead_code)]
fn main() {
    // let p1 = Point::new(10, 10);
    // let p2 = Point::new(14, 13);

    // if round_two_digits(p1.dist(p2)) == 5.0 {
    //     println!(true)
    // }
}
