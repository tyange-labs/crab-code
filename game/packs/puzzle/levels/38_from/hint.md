impl From<Color> for Digit { fn from(color: Color) -> Digit { match color { Color::Red => Digit(1), Color::Blue => Digit(2) } } }
