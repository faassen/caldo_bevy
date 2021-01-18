use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
mod data;
mod renderplugin;
use bevy_rapier2d::physics::{
    JointBuilderComponent, RapierConfiguration, RapierPhysicsPlugin, RigidBodyHandleComponent,
};
use bevy_rapier2d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier2d::rapier::geometry::ColliderBuilder;
use data::{Cell, Instr, Processor};
use na::{Point2, Rotation2, Vector2};
use nalgebra as na;
use rapier2d::dynamics::RigidBodySet;
use rapier2d::dynamics::{BallJoint, FixedJoint, PrismaticJoint};
use rapier2d::math::Vector;

enum Side {
    North,
    East,
    South,
    West,
}

struct Thruster {
    side: Side,
    on: bool,
}

fn setup_physics(commands: &mut Commands) {
    // Static rigid-body with a cuboid shape.
    let rigid_body1 = RigidBodyBuilder::new_static().rotation(0.2);
    let collider1 = ColliderBuilder::cuboid(10.0, 1.0);
    commands.spawn((rigid_body1, collider1));

    let a_body = RigidBodyBuilder::new_dynamic()
        .translation(0.0, 50.0)
        .rotation(3.2);
    let a_collider = ColliderBuilder::cuboid(1.0, 1.0);
    let a_entity = commands
        .spawn((
            a_body,
            a_collider,
            Thruster {
                side: Side::South,
                on: true,
            },
        ))
        .current_entity()
        .unwrap();

    let b_body = RigidBodyBuilder::new_dynamic().translation(4.0, 50.0);
    let b_collider = ColliderBuilder::cuboid(1.0, 1.0).friction(0.0);
    let b_entity = commands
        .spawn((
            b_body,
            b_collider,
            Thruster {
                side: Side::West,
                on: true,
            },
        ))
        .current_entity()
        .unwrap();

    // let joint = BallJoint::new(Point2::new(1.0, 0.0), Point2::new(-1.0, 0.0));
    // commands.spawn((JointBuilderComponent::new(joint, a_entity, b_entity),));
    // // Dynamic rigid-body with cube shape.
    // let iter = 0..10;
    // iter.for_each(|item| {
    //     let rigid_body2 = RigidBodyBuilder::new_dynamic().translation((item as f32) * 2.0, 50.0);
    //     let collider2 = ColliderBuilder::cuboid(1.0, 1.0);
    //     commands.spawn((rigid_body2, collider2));
    // });
    // let iter = 0..10;
    // iter.for_each(|item| {
    //     let rigid_body2 = RigidBodyBuilder::new_dynamic()
    //         .translation((item as f32) * 2.0, 55.0)
    //         .mass(100.0, true);
    //     let collider2 = ColliderBuilder::ball(1.0).friction(0.).restitution(1.0);
    //     commands.spawn((rigid_body2, collider2));
    // });
    // let iter = 0..10;
    // iter.for_each(|item| {
    //     let rigid_body2 = RigidBodyBuilder::new_dynamic()
    //         .translation((item as f32) * 3.0, 55.0 + (item as f32 * 3.0))
    //         .mass(1000.0, true);
    //     let collider2 = ColliderBuilder::cuboid(1.5, 1.5)
    //         .friction(0.)
    //         .restitution(1.0);
    //     commands.spawn((rigid_body2, collider2));
    // });
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

fn thruster_system(
    mut bodies: ResMut<RigidBodySet>,
    query: Query<(&RigidBodyHandleComponent, &Thruster)>,
) {
    for (rigid_body_handle, thruster) in query.iter() {
        let body = bodies.get_mut(rigid_body_handle.handle()).unwrap();
        let t = body.position();
        // let rotation = pos.rotation;

        // let matrix = rotation.to_rotation_matrix();
        let v = match thruster.side {
            Side::North => Vector2::new(0.0, -0.1),
            Side::East => Vector2::new(-0.1, 0.0),
            Side::South => Vector2::new(0.0, 0.1),
            Side::West => Vector2::new(0.1, 0.0),
        };

        let r = t * v;

        // let r = v * matrix;

        body.apply_impulse(r, true);
    }
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
        .add_plugin(ShapePlugin)
        // winit window and input backend for Bevy (?)
        .add_plugin(bevy_winit::WinitPlugin::default())
        // wgpu backend for Bevy (?)
        .add_plugin(bevy_wgpu::WgpuPlugin::default())
        // enable Rapier physics
        .add_plugin(RapierPhysicsPlugin)
        // our own render plugin, based on Rapier's for now
        .add_plugin(renderplugin::RapierRenderPlugin)
        .add_resource(RapierConfiguration {
            gravity: Vector::new(0.0, 0.0),
            ..Default::default()
        })
        // set up graphics
        .add_startup_system(setup_graphics.system())
        // setup physics
        .add_startup_system(setup_physics.system())
        .add_system(thruster_system.system())
        .run();
}
