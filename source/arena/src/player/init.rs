use bevy::prelude::*;
use bevy_loading::Progress;
use zombrr_core::{ArenaOptions, ZombrrObject};

use super::cameras::PlayerCamerasBundle;
use crate::controllers::character::SpawnCharacter;
use crate::controllers::weapon::SpawnWeapon;

pub fn init_player(
    mut commands: Commands,
    options: Res<ArenaOptions>,
    mut spawn_characters: EventWriter<SpawnCharacter>,
    mut spawn_weapons: EventWriter<SpawnWeapon>,
    query: Query<(&Name, &Transform), With<zombrr_map::ArenaGltfMapObject>>,
) -> Progress {
    for (name, transform) in query.iter() {
        if name.as_str() == "PlayerSpawn" {
            let entity = commands
                .spawn_bundle(super::PlayerBundle::from(*transform))
                .with_children(|parent| {
                    PlayerCamerasBundle::spawn(parent, Transform::from_xyz(0.0, 2.0, 0.7));
                })
                .insert(ZombrrObject::Player)
                .id();
            spawn_characters.send(SpawnCharacter {
                parent: entity,
                character: options.player.character.clone(),
            });
            spawn_weapons.send(SpawnWeapon {
                parent: entity,
                weapon: options.player.weapon.clone(),
            });
            return Progress { done: 1, total: 1 };
        }
    }
    Progress { done: 0, total: 1 }
}
