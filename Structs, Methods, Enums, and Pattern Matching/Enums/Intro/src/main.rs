enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => std::f64::consts::PI * (radius * radius),
            Shape::Rectangle(length, height) => length * height,
            Shape::Square(side) => side * side,
        }
    }

    fn get_info(&self) -> String {
        match self {
            Shape::Circle(radius) => {
                format!("This object is a circle with a radius of {:.2}", radius)
            },
            Shape::Rectangle(length, height) => {
                format!("This object is a rectangle with a length of {:.2} and width of {:.2}", length, height)
            },
            Shape::Square(side) => {
                format!("This object is a square with a side length of: {:.2} \n", side)
            }
        }
    }
}

//practice with emus - empty, unit, tuple, struct, etc.

fn main() {
    let circle = Shape::Circle(10.0);

    println!("The area of your circle is: {:.2}", circle.area());
    println!("{}", circle.get_info());

    let rect = Shape::Rectangle(3.0, 2.0);
    println!("The area of your rectangle is: {:.2}", rect.area());
    println!("{}", rect.get_info());

    let square = Shape::Square(5.0);
    println!("{}", square.get_info());
}
