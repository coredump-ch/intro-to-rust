pub trait HasArea {
  fn area(&self) -> f64;
}

pub struct Circle {
  x: f64,
  y: f64,
  radius: f64,
}

impl HasArea for Circle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }
}

pub struct Square {
  x: f64,
  y: f64,
  side: f64,
}

impl HasArea for Square {
  fn area(&self) -> f64 {
    self.side * self.side
  }
}

pub fn print_area<Shape: HasArea>(shape: &Shape) {
  println!("{}", shape.area());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        print_area(&Circle{x: 1.0, y: 2.0, radius: 3.0}); // Shape is Circle
        print_area(&Square{x: 4.0, y: 5.0, side: 5.0}); // Shape is Square
    }
}
