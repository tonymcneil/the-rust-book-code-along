fn main() {
    println!("Generics...");

    println!("Generic on function param and return type...");

    let list_i32 = vec![1, 2, 3, 4];
    let item = first(&list_i32);
    println!("first i32 item: {}", item);

    let list_f64 = vec![1.1, 2.2, 3.3, 4.4];
    let item = first(&list_f64);
    println!("first f64 item: {}", item);

    let list_char = vec!['a', 'b', 'c', 'd'];
    let item = first(&list_char);
    println!("first char item: {}", item);
    println!("----------------------------------\n");

    println!("Generic on struct (single type)...");
    let point_i32 = PointSingleType { x: 5, y: 10 };
    let point_f64 = PointSingleType { x: 1.0, y: 4.0 };
    println!("point_i32: {:?}", point_i32);
    println!("point_f64: {:?}", point_f64);
    println!("----------------------------------\n");

    println!("Generic on struct (multiple types)...");
    let point_multi_type = PointMultiType { x: 5, y: 10.0 };
    println!("point_multi_type: {:?}", point_multi_type);
    println!("----------------------------------\n");

    println!("Generic method implemented for a struct...");
    let point = Point { x: 1, y: 2 };
    println!("point.tuple(): {:?}", point.tuple());
    println!("----------------------------------\n");

    println!("Method of specific type implemented for a struct...");
    let point = Point { x: 1.5, y: 2.5 };
    println!(
        "point.distance_from_origin(): {:?}",
        point.distance_from_origin()
    );
    println!("----------------------------------\n");

    println!("Method using different generic types from its struct’s definition...");
    let p1 = PointMultiType { x: 5, y: 10.4 };
    let p2 = PointMultiType { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    println!("----------------------------------\n");
}

/// get the first item in a list of a generic type
fn first<T>(list: &[T]) -> &T {
    &list[0]
}

#[derive(Debug)]
struct PointSingleType<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointMultiType<T, U> {
    x: T,
    y: U,
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn tuple(&self) -> (&T, &T) {
        (&self.x, &self.y)
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> PointMultiType<T, U> {
    fn mixup<V, W>(self, other: PointMultiType<V, W>) -> PointMultiType<T, W> {
        PointMultiType {
            x: self.x,
            y: other.y,
        }
    }
}
