#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

extern crate rocket;
mod lib;
use lib::direction::Direction;
use lib::point::Point;
use lib::robot::Robot;

static mut ROBOT: Robot = Robot {
    facing: Direction::North,
    position: Point { x: 0, y: 0 },
};

#[get("/")]
fn index() -> String {
    unsafe { serde_json::to_string(&ROBOT).unwrap() }
}

#[get("/move")]
fn move_forward() -> String {
    unsafe {
        ROBOT.move_forward();
        serde_json::to_string(&ROBOT).unwrap()
    }
}

#[get("/right")]
fn right() -> String {
    unsafe { serde_json::to_string(&ROBOT.turn_right()).unwrap() }
}

#[get("/left")]
fn left() -> String {
    unsafe { serde_json::to_string(&ROBOT.turn_left()).unwrap() }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, move_forward, right, left])
        .launch();
}
