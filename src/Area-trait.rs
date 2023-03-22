trait Area {
    fn area(&self) -> f64;
}

// 定义圆形结构体
struct Circle {
    radius: f64,
}

// 为圆形实现Area trait
impl Area for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 定义三角形结构体
struct Triangle {
    base: f64,
    height: f64,
}

// 为三角形实现Area trait
impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 定义正方形结构体
struct Square {
    side: f64,
}

// 为正方形实现Area trait
impl Area for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 定义一个泛型函数，接受参数，并打印其面积
fn print_area<T: Area>(shape: &T) {
    println!("面积: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 3.0, height: 4.0 };
    let square = Square { side: 4.0 };

    print_area(&circle);
    print_area(&triangle);
    print_area(&square);
}
