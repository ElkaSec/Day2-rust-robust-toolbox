trait Shape {
    fn name(&self) -> &'static str;
    fn area(&self) -> f64;
}
fn describe(shape: &impl Shape) {
    println!("{} -> {}", &shape.name(), &shape.area());
}

struct Cercle {
    r: f64,
}
struct Square {
    w: f64,
}

struct Rect {
    w: f64,
    h: f64,
}
impl Shape for Cercle {
    fn name(&self) -> &'static str {
        "cercle"
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * self.r * self.r
    }
}
impl Shape for Square {
    fn name(&self) -> &'static str {
        "Square"
    }

    fn area(&self) -> f64 {
        self.w * self.w
    }
}

impl Shape for Rect {
    fn name(&self) -> &'static str {
        "Rectangle"
    }

    fn area(&self) -> f64 {
        self.w * self.h
    }
}

fn main() {
    let cer: Cercle = Cercle { r: 5.0 };
    let rec: Rect = Rect { w: 5.0, h: 3.0 };
    let squ: Square = Square { w: 5.0 };

    describe(&cer);
    describe(&squ);
    describe(&rec);
}
