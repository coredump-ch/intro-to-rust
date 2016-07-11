pub enum Color {
  Red,
  Green,
  Blue,
}

pub enum Shape {
  Circle{radius: f64},
  Square{side: f64},
}


pub fn color_to_string(color: Color) -> String {
  match color {
    Color::Red => "red",
    Color::Green => "green",
    Color::Blue => "blue",
  }.into()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_color_to_string() {
    assert_eq!(color_to_string(Color::Red), "red".to_string());
  }

  #[test]
  fn match_shape() {
    let shape = Shape::Circle{radius: 1.0};
    match shape {
      Shape::Circle{radius} => {
        println!("Circle with radius {}", radius)
      }
      _ => panic!("Must be a circle!"),
    }
  }
}
