use std::convert::TryInto;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;
mod data;
mod renderplugin;
use bevy_rapier2d::physics::{
    ColliderHandleComponent, EventQueue, JointBuilderComponent, RapierConfiguration,
    RapierPhysicsPlugin, RigidBodyHandleComponent,
};
use bevy_rapier2d::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use bevy_rapier2d::rapier::geometry::{ColliderBuilder, ColliderSet};
use data::{Cell, Instr, Processor};
use na::{Point2, Rotation2, Vector2};
use nalgebra as na;

use rand::Rng;
use rapier2d::dynamics::{BallJoint, FixedJoint, PrismaticJoint};
use rapier2d::geometry::ContactEvent;
use rapier2d::math::Vector;

struct Thruster {
    side: u8,
    on: bool,
}

fn regular_polygon(sides: usize, radius: f32) -> Vec<Point2<f32>> {
    use std::f32::consts::PI;
    let n = sides as f32;
    let internal = (n - 2.0) * PI / n;
    let offset = -internal / 2.0;

    let mut points: Vec<Point2<f32>> = Vec::with_capacity(sides);
    let step = 2.0 * PI / n;

    for i in 0..sides {
        let cur_angle = (i as f32).mul_add(step, offset);
        let x = radius.mul_add(cur_angle.cos(), 0.0);
        let y = radius.mul_add(cur_angle.sin(), 0.0);
        points.push(Point2::new(x, y));
    }
    points
}

fn setup_physics(commands: &mut Commands) {
    // Static rigid-body with a cuboid shape.
    let rigid_body1 = RigidBodyBuilder::new_static().rotation(0.2);
    let collider1 = ColliderBuilder::cuboid(10.0, 1.0);
    commands.spawn((rigid_body1, collider1));

    // let a_body = RigidBodyBuilder::new_dynamic()
    //     .translation(0.0, 50.0)
    //     .rotation(3.2);
    // let a_collider = ColliderBuilder::cuboid(1.0, 1.0);
    // let a_entity = commands
    //     .spawn((
    //         a_body,
    //         a_collider,
    //         Thruster {
    //             side: Side::North,
    //             on: true,
    //         },
    //     ))
    //     .current_entity()
    //     .unwrap();

    // let b_body = RigidBodyBuilder::new_dynamic().translation(4.0, 50.0);
    // let b_collider = ColliderBuilder::cuboid(1.0, 1.0).friction(0.0);
    // let b_entity = commands
    //     .spawn((
    //         b_body,
    //         b_collider,
    //         Thruster {
    //             side: Side::West,
    //             on: true,
    //         },
    //     ))
    //     .current_entity()
    //     .unwrap();

    // let d_body = RigidBodyBuilder::new_dynamic().translation(5.0, 35.0);
    // let d_points = regular_polygon(6, 2.0);
    // let d_collider = ColliderBuilder::convex_hull(&d_points).unwrap();
    // commands.spawn((d_body, d_collider, Thruster { side: 1, on: true }));

    let c_body = RigidBodyBuilder::new_dynamic()
        .translation(7.0, 45.0)
        .mass(2.0);
    let c_points = regular_polygon(6, 1.0);
    let c_collider = ColliderBuilder::convex_hull(&c_points).unwrap();
    commands.spawn((c_body, c_collider, Thruster { side: 5, on: true }));

    let iter = 0..40;
    let points = regular_polygon(6, 1.0);

    let mut rng = rand::thread_rng();

    iter.for_each(|item| {
        let body = RigidBodyBuilder::new_dynamic().translation(
            rng.gen::<f32>() * 50.0 - 25.0,
            rng.gen::<f32>() * 50.0 - 25.0,
        );
        let collider = ColliderBuilder::convex_hull(&points).unwrap();
        commands.spawn((
            body,
            collider,
            Thruster {
                side: rng.gen_range(0..6),
                on: true,
            },
        ));
    })

    // let joint = BallJoint::new(Point2::new(1.0, 0.0), Point2::new(-1.0, 0.0));
    // commands.spawn((JointBuilderComponent::new(joint, a_entity, b_entity),));
    // Dynamic rigid-body with cube shape.

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

fn setup_user_data(
    mut colliders: ResMut<ColliderSet>,
    query: Query<(Entity, &ColliderHandleComponent)>,
) {
    for (entity, collider_handle) in &mut query.iter() {
        if let Some(collider) = colliders.get_mut(collider_handle.handle()) {
            collider.user_data = entity.to_bits() as u128;
            // println!("set user data! {}", entity.to_bits())
        }
    }
}

fn display_events(colliders: Res<ColliderSet>, events: Res<EventQueue>) {
    while let Ok(contact_event) = events.contact_events.pop() {
        match contact_event {
            ContactEvent::Started(first_handle, second_handle) => {
                if let Some(collider) = colliders.get(first_handle) {
                    println!("Received contact event");
                    let first_entity = Entity::from_bits(collider.user_data as u64);
                    println!("User data {:?}", collider.user_data);
                    println!("First entity: {:?}", first_entity);
                }
            }
            _ => {}
        }
    }
}

fn setup_graphics(commands: &mut Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.scale = 10.0;
    // not sure why these two need to be configured
    commands
        // .spawn(LightBundle {
        //     transform: Transform::from_translation(Vec3::new(1000.0, 100.0, 2000.0)),
        //     ..Default::default()
        // })
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

        let impulse: f32 = 0.1;
        let v = vector_for_side(6, thruster.side);
        let r = t * v * impulse;

        body.apply_impulse(r, true);
    }
}

fn vector_for_side(sides: u8, s: u8) -> Vector2<f32> {
    use std::f32::consts::PI;
    // adjust half PI to get it to point up, as up side is side 0,
    // then counting clockwise
    radian_to_vector(((2. * PI) / (sides as f32)) * (s as f32) - 0.5 * PI)
}

fn radian_to_vector(r: f32) -> Vector2<f32> {
    // not sure why I have to flip the y coordinate
    Vector2::new(r.cos(), -r.sin())
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
        .add_system(setup_user_data.system())
        .add_system(thruster_system.system())
        .add_system(display_events.system())
        .run();
}

#[macro_use]
extern crate assert_float_eq;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radian_to_vector() {
        use std::f32::consts::PI;

        let v = radian_to_vector(0.0);
        assert_float_absolute_eq!(v.x, 1.0);
        assert_float_absolute_eq!(v.y, 0.0);

        let v2 = radian_to_vector(0.5 * PI);
        assert_float_absolute_eq!(v2.x, 0.0);
        assert_float_absolute_eq!(v2.y, -1.0);

        let v3 = radian_to_vector(PI);
        assert_float_absolute_eq!(v3.x, -1.0);
        assert_float_absolute_eq!(v3.y, 0.0);

        let v4 = radian_to_vector(1.5 * PI);
        assert_float_absolute_eq!(v4.x, 0.0);
        assert_float_absolute_eq!(v4.y, 1.0);
    }

    #[test]
    fn test_vector_for_side() {
        let v0 = vector_for_side(6, 0);
        assert_float_absolute_eq!(v0.x, 0.0);
        assert_float_absolute_eq!(v0.y, 1.0);

        let v1 = vector_for_side(6, 1);
        assert_float_absolute_eq!(v1.x, 0.8660254);
        assert_float_absolute_eq!(v1.y, 0.5);

        let v2 = vector_for_side(6, 2);
        assert_float_absolute_eq!(v2.x, 0.8660254);
        assert_float_absolute_eq!(v2.y, -0.5);

        let v3 = vector_for_side(6, 3);
        assert_float_absolute_eq!(v3.x, 0.0);
        assert_float_absolute_eq!(v3.y, -1.0);

        let v4 = vector_for_side(6, 4);
        assert_float_absolute_eq!(v4.x, -0.8660254);
        assert_float_absolute_eq!(v4.y, -0.5);

        let v5 = vector_for_side(6, 5);
        assert_float_absolute_eq!(v5.x, -0.8660254);
        assert_float_absolute_eq!(v5.y, 0.5);
    }
}
