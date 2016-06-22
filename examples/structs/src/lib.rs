pub struct Point {
  x: f64,
  y: f64,
}

impl Point {
  /// static method new() ("constructor" per convention)
  pub fn new() -> Point {
    Point{x:0.0, y:0.0}
  }

  pub fn distance_from_zero(&self) -> f64 {
    (self.x*self.x + self.y*self.y).sqrt()
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
      let p = super::Point::new();
      assert_eq!(p.x, 0.0);
      assert_eq!(p.y, 0.0);
    }

    #[test]
    fn test_initialization() {
      let p = super::Point{x:1.0, y:2.0};
      assert_eq!(p.x, 1.0);
      assert_eq!(p.y, 2.0);
    }

    #[test]
    fn test_distance_from_zero() {
      let p = super::Point{x:3.0, y:4.0};
      assert_eq!(p.distance_from_zero(), 5.0);
    }

}
