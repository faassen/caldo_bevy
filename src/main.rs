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
    let rigid_body1 = RigidBodyBuilder::new_static().rotation(0.2);
    let collider1 = ColliderBuilder::cuboid(10.0, 1.0);
    commands.spawn((rigid_body1, collider1));

    // Dynamic rigid-body with ball shape.
    let rigid_body2 = RigidBodyBuilder::new_dynamic().translation(0.0, 50.0);
    let collider2 = ColliderBuilder::ball(3.0);
    commands.spawn((rigid_body2, collider2));
}

fn setup_graphics(commands: &mut Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 10.0;
    // not sure why these two need to be configured
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
        // the background color
        .add_resource(ClearColor(Color::rgb(
            0xF9 as f32 / 255.0,
            0xF9 as f32 / 255.0,
            0xFF as f32 / 255.0,
        )))
        // Some kind of anti aliassing?
        .add_resource(Msaa::default())
        // default bevy plugins. Required to make physics work
        .add_plugins(DefaultPlugins)
        // winit window and input backend for Bevy (?)
        .add_plugin(bevy_winit::WinitPlugin::default())
        // wgpu backend for Bevy (?)
        .add_plugin(bevy_wgpu::WgpuPlugin::default())
        // enable Rapier physics
        .add_plugin(RapierPhysicsPlugin)
        // our own render plugin, based on Rapier's for now
        .add_plugin(renderplugin::RapierRenderPlugin)
        // set up graphics
        .add_startup_system(setup_graphics.system())
        // setup physics
        .add_startup_system(setup_physics.system())
        .run();
}
