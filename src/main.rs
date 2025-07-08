// https://bevy.org/learn/quick-start/getting-started/apps/

#![allow(dead_code)]

use bevy::prelude::*;

#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("hello {}!", name.0);
    }
}

fn hello_system() {
    println!("Hello, world!");
}

struct Entity(u64);

fn main() {
    App::new()
        .add_systems(Startup, add_people)
        .add_systems(Update, (hello_system, greet_people))
        .run();
}
