use bevy::prelude::*;

mod data;

struct Position {
    x: u64,
    y: u64,
}

fn print_position_system(query: Query<&Position>) {
    for position in query.iter() {
        println!("position: {:?} {:?}", position.x, position.y);
    }
}

fn hello_world() {
    println!("hello world!");
}

#[bevy_main]
fn main() {
    App::build().add_system(hello_world.system()).run();
}
