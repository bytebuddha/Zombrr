use crate::gltf::GltfExtras;
use bevy_asset::prelude::*;
use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_loading::Progress;
use bevy_log::prelude::*;
use bevy_math::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_render::prelude::*;
use bevy_scene::prelude::*;
use bevy_transform::prelude::*;

pub fn init_map_objects(
    mut commands: Commands,
    resources: Option<Res<crate::ArenaMapData>>,
    scene_spawner: Res<SceneSpawner>,
    meshes: Res<Assets<Mesh>>,
    root_elements: Query<Entity, Without<Parent>>,
    parents: Query<(&GltfExtras, &Children, Option<&Name>), With<crate::ArenaGltfMapObject>>,
    children_query: Query<(Entity, &Handle<Mesh>), With<crate::ArenaGltfMapObject>>,
) -> Progress {
    let mut progress = 0;
    if let Some(map_data) = &resources {
        // Update The Gltf Entities.
        if let Some(entity_iter) = scene_spawner.iter_instance_entities(map_data.instance_id) {
            entity_iter.for_each(|entity| {
                if let Ok(root) = root_elements.get(entity) {
                    commands
                        .entity(root)
                        .insert(Name::new(format!("Arena Map({})", map_data.name)))
                        .insert(crate::ArenaMapRoot);
                }
                if let Ok((extras, children, name)) = parents.get(entity) {
                    for child in children.iter() {
                        let node_name = name
                            .map(|x| x.to_string())
                            .unwrap_or(format!("{:?}", child));
                        if let Ok((entity, mesh)) = children_query.get(*child) {
                            let mut builder = commands.entity(entity);
                            if let Some(rb) = extras.rigid_body.as_ref() {
                                match serde_json::from_str::<RigidBodyType>(&format!("\"{}\"", rb))
                                {
                                    Err(err) => {
                                        warn!("Failed to deserialize extras field: {}", err)
                                    }
                                    Ok(body_type) => {
                                        builder.insert_bundle(RigidBodyBundle {
                                            body_type,
                                            ..Default::default()
                                        });
                                    }
                                }
                            }
                            if let Some(c_type) = extras.collider.as_ref() {
                                match serde_json::from_str::<ColliderType>(&format!(
                                    "\"{}\"",
                                    c_type
                                )) {
                                    Err(err) => warn!(
                                        "Failed to deserialize extras field for node {}: {}",
                                        node_name, err
                                    ),
                                    Ok(collider_type) => {
                                        builder
                                            .insert_bundle(ColliderBundle {
                                                collider_type,
                                                shape: mesh_collider_shape(
                                                    meshes.get(mesh).unwrap(),
                                                ),
                                                mass_properties: ColliderMassProps::Density(400.0),
                                                ..Default::default()
                                            })
                                            .insert(bevy_hilt::prelude::HiltDebugCollider {
                                                color: Color::ORANGE,
                                            })
                                            .insert(zombrr_damage::BulletHoles::default());
                                    }
                                }
                            }
                            if let Some(debug_color) = extras.debug_color.as_ref() {
                                match serde_json::from_str::<zombrr_core::packages::Color>(
                                    &format!("\"{}\"", debug_color),
                                ) {
                                    Err(err) => warn!(
                                        "Failed to deserialize extras field for node {}: {}",
                                        node_name, err
                                    ),
                                    Ok(debug_color) => {
                                        builder.insert(bevy_hilt::prelude::HiltDebugCollider {
                                            color: crate::zombrr_color_to_bevy(&debug_color),
                                        });
                                    }
                                }
                            }
                        }
                    }
                } else {
                    commands
                        .entity(entity)
                        .insert_bundle((crate::ArenaGltfMapObject,));
                }
            });
            progress = 1;
        }
    }
    Progress {
        done: progress,
        total: 1,
    }
}

fn mesh_collider_shape(mesh: &Mesh) -> ColliderShape {
    use bevy_rapier3d::na::Point3;

    let mut vertices: Vec<Point3<f32>> = Vec::new();
    let mut indices: Vec<[u32; 3]> = Vec::new();

    match mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
        bevy_render::mesh::VertexAttributeValues::Float3(attr) => {
            for pos in attr.iter() {
                vertices.push(Vec3::new(pos[0], pos[1], pos[2]).into());
            }
        }
        _ => {}
    }

    match mesh.indices().unwrap() {
        bevy_render::mesh::Indices::U32(ind) => {
            for i in 0..(ind.len() / 3) {
                indices.push([ind[i * 3], ind[i * 3 + 1], ind[i * 3 + 2]]);
            }
        }
        _ => {}
    }

    ColliderShape::trimesh(vertices, indices)
}
