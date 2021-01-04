use bevy::prelude::*;

mod data;
mod renderplugin;
use bevy_rapier2d::physics::{RapierConfiguration, RapierPhysicsPlugin};
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier2d::rapier::geometry::ColliderBuilder;

// use bevy_rapier2d::render::RapierRenderPlugin;

use data::{Cell, Instr, Processor};

struct Position {
    x: u64,
    y: u64,
}

struct Name {
    name: String,
}

// we could detect whether position has changed
// if position changes, we update the neighbors components of these
// places

//      A
//      B C
//    E D

fn setup_entities(commands: &mut Commands) {
    commands.spawn((Position { x: 10, y: 10 }, Name { name: "A".into() }));
    commands.spawn((Position { x: 10, y: 11 }, Name { name: "B".into() }));
    commands.spawn((Position { x: 11, y: 11 }, Name { name: "C".into() }));
    commands.spawn((Position { x: 10, y: 12 }, Name { name: "D".into() }));
    commands.spawn((Position { x: 9, y: 12 }, Name { name: "E".into() }));
}

// fn update_position_map(mut position_map: ResMut<PositionMap>, query: Query<(Entity, &Position)>) {
//     for (entity, position) in query.iter() {
//         position_map.add(entity, (position.x, position.y));
//     }
// }

// fn print_neighbor_system(position_map: Res<PositionMap>, query: Query<(Entity, &Name, &Position)>) {
//     println!("Print neighbor system");
//     for (entity, name, position) in query.iter() {
//         println!(
//             "entity {:?}, name {} position: {:?} {:?}",
//             entity, name.name, position.x, position.y
//         );
//         println!(
//             "Neighbor entities {:?}",
//             position_map.get_neighbors((position.x, position.y))
//         )
//     }
// }

// to efficiently render part of a huge world we need a good
// space partitioning system
// can this be used to make neighborhood checks more efficient too?

// we can keep track of which things are in which partition by
// tracking position changes, but can see before the changes then?

fn setup_physics(commands: &mut Commands) {
    // Static rigid-body with a cuboid shape.
    let rigid_body1 = RigidBodyBuilder::new_static();
    let collider1 = ColliderBuilder::cuboid(10.0, 1.0);
    commands.spawn((rigid_body1, collider1));

    // Dynamic rigid-body with ball shape.
    let rigid_body2 = RigidBodyBuilder::new_dynamic().translation(0.0, 50.0);
    let collider2 = ColliderBuilder::cuboid(1.0, 1.0);
    commands.spawn((rigid_body2, collider2));
}

fn setup_graphics(commands: &mut Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 10.0;

    commands
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
            ..Default::default()
        })
        .spawn(Camera2dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Camera2dBundle::default()
        });
}

#[bevy_main]
fn main() {
    App::build()
        .add_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        .add_resource(Msaa::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_winit::WinitPlugin::default())
        .add_plugin(bevy_wgpu::WgpuPlugin::default())
        .add_plugin(RapierPhysicsPlugin)
        .add_plugin(renderplugin::RapierRenderPlugin)
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .run();
}
