#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point {
            x: ax,
            y: ay,
        },
        bottom_right: Point {
            x: bx,
            y: by,
        }
    } = rectangle;
    ((bx - ax) * (by - ay)).abs()
}

fn main() {

    let rectangle = Rectangle { 
        top_left: Point { x: 0.0, y: 5.0 },  
        bottom_right:  Point { x: 10.0, y: 0.0 } 
    };
    println!("rectangle: {:?}, area: {:?}", rectangle, rect_area(&rectangle));

}
