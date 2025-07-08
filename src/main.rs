// https://bevy.org/learn/quick-start/getting-started/apps/

#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}

fn hello_system() {
    println!("Hello, world!");
}

struct Entity(u64);

fn main() {
    App::new().add_systems(Update, hello_system).run();
}
