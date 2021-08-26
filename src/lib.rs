#![feature(test)]

use std::ops::{Add, Sub};
extern crate test;

const ATOL: f64 = 1e-8;
const RTOL: f64 = 1e-5;

/// A point in 2D space. Can also be thought of as a 2D vector
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// What is the angle (in radians) from the positive x-axis to the point. I.e. the
    /// angle if this point were converted to polar coordinates
    /// ```
    /// let p = rsgeo::Point{x: 1.0, y: 1.0};
    /// let a = p.angle();
    /// assert_eq!(a, std::f64::consts::PI)
    /// ```
    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }

    /// Multiply a point by a scalar
    /// ```
    /// let p = rsgeo::Point{x: 1.0, y: 1.0};
    /// let result = p.mul(3.0);
    /// assert_eq!(result, rsgeo::Point{x: 3.0, y: 3.0})
    ///```
    pub fn mul(&self, x: f64) -> Point {
        Point {
            x: self.x * x,
            y: self.y * x,
        }
    }

    /// Divide a point by a scalar
    /// ```
    /// let p = rsgeo::Point{x: 3.0, y: 3.0};
    /// let result = p.div(3.0);
    /// assert_eq!(result, rsgeo::Point{x: 1.0, y: 1.0})
    ///```
    pub fn div(&self, x: f64) -> Point {
        Point {
            x: self.x / x,
            y: self.y / x,
        }
    }

    /// rotate will rotate the point about the origin.
    pub fn rotate(&self, angle: f64) -> Point {
        let s = angle.sin();
        let c = angle.cos();
        Point {
            x: (self.x * c) - (self.y * s),
            y: (self.x * s) + (self.y * c),
        }
    }

    /// Check if two points are close to eachother
    pub fn isclose(&self, other: Point) -> bool {
        f64_isclose(self.x, other.x) && f64_isclose(self.y, other.y)
    }

    /// xintercept will calculate the x-intercept of an infinite line, as defined by the
    /// two points `self` and `other`. If the line is horizontal, returns Inf.
    pub fn xintercept(&self, other: &Point) -> f64 {
        let i = self.x - (self.y * (other.x - self.x) / (other.y - self.y));
        if f64::is_infinite(i) {
            f64::INFINITY
        } else {
            i
        }
    }

    /// magnitude computes the magnitude of the vector
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// normalize will normalize a point to unit magnitude
    pub fn normalize(&self) -> Point {
        self.div(self.magnitude())
    }

    /// Compute the dot product of two Points
    pub fn dot_product(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineSegment {
    p1: Point,
    p2: Point,
}

impl LineSegment {
    pub fn isclose(&self, other: &LineSegment) -> bool {
        self.p1.isclose(other.p1) && self.p2.isclose(other.p2)
    }
}

fn f64_isclose(a: f64, b: f64) -> bool {
    (a - b).abs() <= (ATOL + (RTOL * b.abs()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_angle1() {
        let p = Point { x: 1.0, y: 1.0 };
        assert_eq!(p.angle(), 1.0_f64.atan2(1.0))
    }

    #[test]
    fn test_add() {
        let p1 = Point { x: 1.0, y: 1.0 };
        let p2 = Point { x: 3.0, y: 10.0 };
        assert_eq!(Point { x: 4.0, y: 11.0 }, p1 + p2)
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        let p = Point { x: 1.0, y: 1.0 };
        b.iter(|| p.angle());
    }

    #[test]
    fn test_rotate_by45deg() {
        let p = Point { x: 1.0, y: 0.0 };
        let angle = std::f64::consts::PI / 4.0;
        let expected = Point {
            x: (std::f64::consts::PI / 4.0).cos(),
            y: (std::f64::consts::PI / 4.0).sin(),
        };
        let got = p.rotate(angle);
        assert!(expected.isclose(got))
    }

    #[bench]
    fn bench_rotate_by45deg(b: &mut Bencher) {
        let p = Point { x: 1.0, y: 0.0 };
        let angle = std::f64::consts::PI / 4.0;
        b.iter(|| p.rotate(angle));
    }

    #[test]
    fn test_rotate_by90deg() {
        let p = Point { x: 1.0, y: 0.0 };
        let angle = std::f64::consts::PI / 2.0;
        let expected = Point { x: 0.0, y: 1.0 };
        let got = p.rotate(angle);
        assert!(expected.isclose(got))
    }

    #[test]
    fn test_xintercept_two_pts_stacked_vertically() {
        let p = Point { x: 1.0, y: -1.0 };
        let q = Point { x: 1.0, y: 1.0 };
        let expected = 1.0;
        let got = p.xintercept(&q);
        assert_eq!(expected, got)
    }

    #[bench]
    fn bench_xintercept_two_pts_stacked_vertically(b: &mut Bencher) {
        let p = Point { x: 1.0, y: -1.0 };
        let q = Point { x: 1.0, y: 1.0 };
        b.iter(|| p.xintercept(&q));
    }

    #[test]
    fn test_xintercept_two_pts_on_y_axis() {
        let p = Point { x: 0.0, y: -1.0 };
        let q = Point { x: 0.0, y: 1.0 };
        let expected = 0.0;
        let got = p.xintercept(&q);
        assert_eq!(expected, got)
    }

    #[bench]
    fn bench_xintercept_two_pts_on_y_axis(b: &mut Bencher) {
        let p = Point { x: 0.0, y: -1.0 };
        let q = Point { x: 0.0, y: 1.0 };
        b.iter(|| p.xintercept(&q));
    }
}
