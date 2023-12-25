mod circle;
mod ellipse;
mod parallelogram;
mod rectangle;
mod regular_polygon;
mod sector;
mod square;
mod trapezoid;
mod triangle;

pub enum Shape {
    Circle,
    Ellipse,
    Parallelogram,
    Rectangle,
    RegularPolygon,
    Sector,
    Square,
    Trapezoid,
    Triangle,
}

pub const SHAPES: [&str; 9] = [
    "Circle",
    "Ellipse",
    "Parallelogram",
    "Rectangle",
    "Regular polygon",
    "Sector",
    "Square",
    "Trapezoid",
    "Triangle",
];

impl Shape {
    pub fn calculate(&self) -> Option<f64> {
        return match self {
            Self::Circle => Some(circle::calculate()),
            Self::Ellipse => Some(ellipse::calculate()),
            Self::Parallelogram => Some(parallelogram::calculate()),
            Self::Rectangle => Some(rectangle::calculate()),
            Self::RegularPolygon => Some(regular_polygon::calculate()),
            Self::Sector => Some(sector::calculate()),
            Self::Square => Some(square::calculate()),
            Self::Trapezoid => Some(trapezoid::calculate()),
            Self::Triangle => triangle::calculate(),
        };
    }
}
