use bevy::prelude::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::physics::{ColliderHandleComponent, RapierConfiguration};
use nalgebra as na;
use rapier2d::dynamics::RigidBodySet;
use rapier2d::geometry::{ColliderSet, ShapeType as RapierShapeType};
use rapier2d::math::Isometry;
use std::collections::HashMap;

/// Plugin responsible for creating meshes to render the Rapier physics scene.
pub struct RapierRenderPlugin;

impl Plugin for RapierRenderPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(stage::PRE_UPDATE, create_collider_renders_system.system());
    }
}

/// The desired render color of a Rapier collider.
pub struct RapierRenderColor(pub f32, pub f32, pub f32);

/// System responsible for attaching a PbrBundle to each entity having a collider.
pub fn create_collider_renders_system(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    configuration: Res<RapierConfiguration>,
    bodies: Res<RigidBodySet>,
    colliders: ResMut<ColliderSet>,
    query: Query<
        (Entity, &ColliderHandleComponent, Option<&RapierRenderColor>),
        Without<Handle<Mesh>>,
    >,
) {
    let ground_color = Color::rgb(
        0xF3 as f32 / 255.0,
        0xD9 as f32 / 255.0,
        0xB1 as f32 / 255.0,
    );

    let palette = [
        Color::rgb(
            0x98 as f32 / 255.0,
            0xC1 as f32 / 255.0,
            0xD9 as f32 / 255.0,
        ),
        Color::rgb(
            0x05 as f32 / 255.0,
            0x3C as f32 / 255.0,
            0x5E as f32 / 255.0,
        ),
        Color::rgb(
            0x1F as f32 / 255.0,
            0x7A as f32 / 255.0,
            0x8C as f32 / 255.0,
        ),
    ];

    let mut icolor = 0;
    let mut body_colors = HashMap::new();

    for (entity, collider, debug_color) in &mut query.iter() {
        if let Some(collider) = colliders.get(collider.handle()) {
            if let Some(body) = bodies.get(collider.parent()) {
                let default_color = if body.is_static() {
                    ground_color
                } else {
                    *body_colors.entry(collider.parent()).or_insert_with(|| {
                        icolor += 1;
                        palette[icolor % palette.len()]
                    })
                };

                let shape = collider.shape();

                let color = debug_color
                    .map(|c| Color::rgb(c.0, c.1, c.2))
                    .unwrap_or(default_color);

                let mesh = match shape.shape_type() {
                    RapierShapeType::Cuboid => Mesh::from(shape::Quad {
                        size: Vec2::new(2.0, 2.0),
                        flip: false,
                    }),
                    RapierShapeType::Ball => Mesh::from(shape::Icosphere {
                        subdivisions: 2,
                        radius: 3.0,
                    }),
                    RapierShapeType::Trimesh => {
                        let mut mesh =
                            Mesh::new(bevy::render::pipeline::PrimitiveTopology::TriangleList);
                        let trimesh = shape.as_trimesh().unwrap();
                        mesh.set_attribute(
                            Mesh::ATTRIBUTE_POSITION,
                            VertexAttributeValues::from(
                                trimesh
                                    .vertices()
                                    .iter()
                                    .map(|vertice| [vertice.x, vertice.y])
                                    .collect::<Vec<_>>(),
                            ),
                        );
                        mesh.set_indices(Some(Indices::U32(
                            trimesh
                                .indices()
                                .iter()
                                .flat_map(|triangle| triangle.iter())
                                .cloned()
                                .collect(),
                        )));
                        mesh
                    }
                    _ => unimplemented!(),
                };

                let scale = match shape.shape_type() {
                    RapierShapeType::Cuboid => {
                        let c = shape.as_cuboid().unwrap();
                        Vec3::new(c.half_extents.x, c.half_extents.y, 1.0)
                    }
                    RapierShapeType::Ball => {
                        let b = shape.as_ball().unwrap();
                        Vec3::new(b.radius, b.radius, b.radius)
                    }
                    RapierShapeType::Trimesh => Vec3::one(),
                    _ => unimplemented!(),
                } * configuration.scale;

                let mut transform = Transform::from_scale(scale);
                sync_transform(
                    collider.position_wrt_parent(),
                    configuration.scale,
                    &mut transform,
                );

                let prim = primitive(
                    materials.add(color.into()),
                    &mut meshes,
                    ShapeType::Ellipse {
                        radius_x: 50.0,
                        radius_y: 50.0,
                    },
                    TessellationMode::Stroke(&StrokeOptions::default().with_line_width(30.0)),
                    Vec3::new(0.0, -50.0, 0.0),
                );

                // let ground_pbr = PbrBundle {
                //     mesh: meshes.add(mesh),
                //     material: materials.add(color.into()),
                //     transform,
                //     ..Default::default()
                // };

                commands.insert(entity, prim);
            }
        }
    }
}

fn sync_transform(pos: &Isometry<f32>, scale: f32, transform: &mut Transform) {
    // Do not touch the 'z' part of the translation, used in Bevy for 2d layering
    transform.translation.x = pos.translation.vector.x * scale;
    transform.translation.y = pos.translation.vector.y * scale;

    let rot = na::UnitQuaternion::new(na::Vector3::z() * pos.rotation.angle());
    transform.rotation = Quat::from_xyzw(rot.i, rot.j, rot.k, rot.w);
}
