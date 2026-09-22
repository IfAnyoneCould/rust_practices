trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn name(&self) -> &'static str;
}

struct Circle {
    radius: f64,
}
struct Rectangle {
    width: f64,
    height: f64,
}
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64 {
        std::f64::consts::PI * 2f64 * self.radius
    }
    fn name(&self) -> &'static str {
        "Circle"
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn perimeter(&self) -> f64 {
        2f64 * (self.width + self.height)
    }
    fn name(&self) -> &'static str {
        "Rectangle"
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height * 0.5f64
    }
    fn perimeter(&self) -> f64 {
        self.base + (0.25f64 * self.base * self.base + self.height * self.height).sqrt() * 2f64
    }
    fn name(&self) -> &'static str {
        "Triangle"
    }
}

fn describe(shape: &dyn Shape) -> String {
    format!(
        "{}: Area: {:.2}, Permimeter: {:.2}",
        shape.name(),
        shape.area(),
        shape.perimeter()
    )
}

fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().fold(0f64, |acc, s| acc + s.area())
}

fn largest(shapes: &[Box<dyn Shape>]) -> Option<&Box<dyn Shape>> {
    if shapes.is_empty() {
        return None;
    }
    Some(
        shapes
            .iter()
            .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
            .unwrap(),
    )
}

fn main() {
    let v: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2f64 }),
        Box::new(Rectangle {
            width: 10f64,
            height: 0.5f64,
        }),
    ];
    println!("total area: {}", total_area(&v));
    match largest(&v) {
        Some(shape) => println!("largest shape: {}", describe(shape.as_ref())),
        None => println!("empty shape vector"),
    }
}
