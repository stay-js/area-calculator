mod circle;
mod ellipse;
mod rectangle;
mod sector;
mod square;
mod trapezoid;
mod triangle;

pub enum Shape {
    Rectangle,
    Square,
    Triangle,
    Trapezoid,
    Circle,
    Sector,
    Ellipse,
    Parallelogram,
}

impl Shape {
    pub fn calculate(&self) {
        let area = match self {
            Self::Rectangle => rectangle::calculate(),
            Self::Square => square::calculate(),
            Self::Triangle => triangle::calculate(),
            Self::Trapezoid => trapezoid::calculate(),
            Self::Circle => circle::calculate(),
            Self::Sector => sector::calculate(),
            Self::Ellipse => ellipse::calculate(),
            Self::Parallelogram => todo!(),
        };

        println!("\nArea: {}", area);
    }
}
