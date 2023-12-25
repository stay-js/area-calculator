mod circle;
mod ellipse;
mod parallelogram;
mod rectangle;
mod sector;
mod square;
mod trapezoid;
mod triangle;

pub enum Shape {
    Circle,
    Ellipse,
    Parallelogram,
    Rectangle,
    Sector,
    Square,
    Trapezoid,
    Triangle,
}

impl Shape {
    pub fn calculate(&self) -> Option<f64> {
        return match self {
            Self::Circle => Some(circle::calculate()),
            Self::Ellipse => Some(ellipse::calculate()),
            Self::Parallelogram => Some(parallelogram::calculate()),
            Self::Rectangle => Some(rectangle::calculate()),
            Self::Sector => Some(sector::calculate()),
            Self::Square => Some(square::calculate()),
            Self::Trapezoid => Some(trapezoid::calculate()),
            Self::Triangle => triangle::calculate(),
        };
    }
}
