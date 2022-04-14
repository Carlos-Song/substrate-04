use std::u32::MAX;

pub enum TrafficLight {
    Red,
    Yello,
    Green,
}

trait Traffic<T> {
    fn exchage(self) -> u8;
}

impl Traffic<TrafficLight> for TrafficLight {
    fn exchage(self) -> u8 {
        match self {
            TrafficLight::Red => 60,
            TrafficLight::Yello => 15,
            TrafficLight::Green => 30,
        }
    }
}

// plus_check_u32 会检查值是否溢出，溢出返回 None
fn plus_check_u32(list: &Vec<u32>) -> Option<u32> {
    let mut count: u32 = 0;
    for &e in list.into_iter() {
        match count.checked_add(e) {
            Some(value) => {
                count += value;
            }
            None => return None,
        };
    }

    Some(count)
}

trait Shape {
    fn area(&self) -> f32;
}

struct Triangle {
    height: f32,
    side: f32,
}

impl Shape for Triangle {
    fn area(&self) -> f32 {
        self.height * self.side
    }
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.height * self.width
    }
}

struct Circle {
    radius: f32,
}

impl Shape for Circle {
    fn area(&self) -> f32 {
        self.radius * 3.14
    }
}

fn get_area<T: Shape>(shape: T) -> f32 {
    shape.area()
}

fn main() {
    // 作业 04
    let red_light = TrafficLight::Red.exchage();
    let green_light = TrafficLight::Yello.exchage();
    let yellow_light = TrafficLight::Green.exchage();

    println!("It's going to last {}s", red_light);
    println!("It's going to last {}s", green_light);
    println!("It's going to last {}s", yellow_light);

    // 作业 05
    let u32_vec = vec![MAX, 10, 20];
    let result = plus_check_u32(&u32_vec);

    match result {
        Some(result) => println!("result is {} ", result),
        None => println!("Overflow the max u32!"),
    }

    // 作业 06
    let circle = Circle { radius: 8.0 };

    let triangle = Triangle {
        side: 8.0,
        height: 4.0,
    };

    let rectangle = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    let circle_area = get_area(circle);
    let triangle_area = get_area(triangle);
    let rectangle_area = get_area(rectangle);

    println!("circle area is {}", circle_area);
    println!("triangle area is {}", triangle_area);
    println!("rectangle area is {}", rectangle_area);
}
