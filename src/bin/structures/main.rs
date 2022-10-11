
#[derive(Debug)]
struct Person {
    name: String, 
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point, 
    bottom_right: Point,
}

fn rect_area(rectangle: Rectangle) -> f32 {
    let Rectangle { top_left: Point {x: ref x1, y: ref y1 }, 
                    bottom_right: Point {x: ref x2, y: ref y2},
                } = rectangle;

    (x2-x1) * (y2-y1)
}

fn square(top_left_corner: &Point, width: f32) -> Rectangle {
    Rectangle {top_left: Point {x: top_left_corner.x, y: top_left_corner.y}, 
               bottom_right: Point {x: top_left_corner.x+width, y: top_left_corner.y+width}
            }
}

fn main(){
    let name = String::from("Peter");
    let age = 27;
    let peter = Person {name, age};

    println!("{:?}", peter);

    let point: Point = Point {x: 10.3, y: 0.4};

    let bottom_right = Point {x: 5.2, ..point};

    let Point {x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge}, 
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let corner = Point {x: 5.2, y: 0.0};
    println!("square {:?}", square(&corner, 1.2));

    println!("area {}", rect_area(_rectangle));
}