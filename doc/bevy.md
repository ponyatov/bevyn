# `bevy`
## simple data-driven game engine
### in [[Rust/Rust|Rust]]

https://github.com/ponyatov/bevyn

- https://bevy.org/learn/quick-start/getting-started/apps/
- https://crates.io/crates/bevy
- https://www.youtube.com/watch?v=XWZBVC2AqsE with [[lyon]]

## [[ECS]]

### [[game/component|component]]

```Rust
#[derive(Component)]
struct Position {
    x: f32,
    y: f32,
}
```

### [[game/system|system]]

just a normal Rust functions:

```Rust
fn print_position_system(query: Query<&Position>) {
    for position in &query {
        println!("position: {} {}", position.x, position.y);
    }
}
```

### [[entity]]

a simple type containing a unique integer: just some object marker

```Rust
struct Entity(u64);
```

# uses
- [[glam]]
- [[wgpu]]
