#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
mod lib;
use lib::direction::Direction;
use lib::robot::Robot;

static mut ROBOT: Robot = Robot {
    facing: Direction::North,
};

#[get("/")]
fn index() -> String {
    unsafe { ROBOT.facing.value().to_string() }
}

#[get("/right")]
fn right() -> String {
    unsafe { ROBOT.turn_right().facing.value().to_string() }
}

#[get("/left")]
fn left() -> String {
    unsafe { ROBOT.turn_left().facing.value().to_string() }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, right, left])
        .launch();
}
