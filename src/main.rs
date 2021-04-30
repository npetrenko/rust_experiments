#![allow(dead_code)]

mod traits;

trait FooTrait {
    fn run(&self) -> i32;
    fn set(&mut self, val: i32);
}

impl FooTrait for i32 {
    fn run(&self) -> i32 {
        *self
    }

    fn set(&mut self, val: i32) {
        *self = val;
    }
}

struct Wrapper {
    value : Box<i32>,
}

fn do_print(b : &i32) {
    // assert_eq!(1, 2);
    println!("Im in function! {}", b);
}

fn lifetime_example<'a>(s: &'a i32) -> &'a i32 {
    s
}

fn overwrite(x : &mut i32) {
    *x = if true {
        0
    } else {
        1
    };
    'outer: loop {
        loop {
            break 'outer;
        }
    }
    *x = 2;

    let uninit;
    loop {
        if true {
            uninit = 0;
            break;
        }
    }
    *x = uninit;

    // uninhabited types:
    // let x: ! = loop {};

    // unreachable!();
}

fn main() {
    // let bx = Box::new(92);
    // println!("Box : {}", bx);
    // let cort = (0,1);
    // println!("Hello, world! {:?} {:?} {:?}"
    //          , &cort as *const (i32, i32)
    //          , &cort.0 as *const i32
    //          , &cort.1 as *const i32);

    // do_print(&*bx);

    let mut vec_box : Vec<Box<dyn FooTrait>> = vec![Box::new(42), Box::new(94)];
    let mut x = 0;
    overwrite(&mut x);
    vec_box[0].set(x);

    for x in &vec_box {
        println!("{}", x.run());
    }

    for i in 0..2 {
        println!("{}", vec_box[i].run());
    }

    for x in &vec_box {
        println!("{}", x.run());
    }
}

struct Kilometers(f64);
struct Tag;

struct Distance<M> {
    amount: f64,
    metric: M,
}

#[derive(Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

fn dom(xs : &[i32; 4]) {
    let p1 = Point {x : 1.0, y : 1.into()};
    let _p2 = Point {x : 2.0, ..p1};
    let _d1: Distance<Tag> = Distance {
        amount: 92.0,
        metric: Tag,
    };
    for x in xs {
        println!("{}", x + 1);
    }
}

enum Shape {
    Circle {
        center: Point,
    },
}

impl Shape {
    fn circle(center: Point, _radius: f64) -> Shape {
        let c = Shape::Circle {center};
        match c {
            Shape::Circle {..} => {
                c
            }
        }
    }
}

enum MyVoid {}

fn do_void(void: MyVoid) -> Vec<Point> {
    match void {
    }
}

fn extract(result: Result<Shape, MyVoid>) -> Shape {
    match result {
        Ok(spam) => spam,
        Err(void) => match void{},
    }
}

enum MyOption<T> {
    Some(T),
    None,
}


// fn do_ub() {
//     let r: &i32;
//     {
//         let x = Box::new(42);
//         r = &x;
//     }
//     println!("Uhu {:?}", r as *const i32);
// }
