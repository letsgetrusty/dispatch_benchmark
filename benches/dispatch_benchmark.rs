use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Define an enum with different variants
enum ShapeEnum {
    Circle(f64),
    Square(f64),
    Triangle(f64, f64, f64),
}

impl ShapeEnum {
    fn area(&self) -> f64 {
        match *self {
            ShapeEnum::Circle(radius) => std::f64::consts::PI * radius * radius,
            ShapeEnum::Square(side) => side * side,
            ShapeEnum::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }
}

// Define a trait for shapes
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

struct Triangle {
    a: f64,
    b: f64,
    c: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
}

// Benchmark function for enum dispatch
fn bench_enum_dispatch(c: &mut Criterion) {
    let shapes = vec![
        ShapeEnum::Circle(10.0),
        ShapeEnum::Square(10.0),
        ShapeEnum::Triangle(10.0, 10.0, 10.0),
    ];

    c.bench_function("enum_dispatch", |b| {
        b.iter(|| {
            for shape in &shapes {
                black_box(shape.area());
            }
        })
    });
}

// Benchmark function for dynamic dispatch
fn bench_dynamic_dispatch(c: &mut Criterion) {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 10.0 }),
        Box::new(Square { side: 10.0 }),
        Box::new(Triangle {
            a: 10.0,
            b: 10.0,
            c: 10.0,
        }),
    ];

    c.bench_function("dynamic_dispatch", |b| {
        b.iter(|| {
            for shape in &shapes {
                black_box(shape.area());
            }
        })
    });
}

criterion_group!(benches, bench_enum_dispatch, bench_dynamic_dispatch);
criterion_main!(benches);
