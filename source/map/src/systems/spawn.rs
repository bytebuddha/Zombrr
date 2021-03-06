use bevy_asset::prelude::*;
use bevy_core::prelude::*;
use bevy_ecs::prelude::*;
use bevy_log::prelude::*;
use bevy_math::prelude::*;
use bevy_pbr::AmbientLight;
use bevy_render::prelude::*;
use bevy_scene::prelude::*;
use chrono::{TimeZone, Utc};
use zombrr_core::packages::{MapData, SkyPreset, Sky};

use bevy_sky::{SkyBundle, SkyMaterial, Sun};

use crate::ArenaMapData;

pub fn spawn_arena_map(
    map: crate::ActiveMap,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut sky_materials: ResMut<Assets<SkyMaterial>>,
) {
    debug!(
        "Spawning Map `{}`\n\t-> Name = {}\n\t-> Path: {:?}\n\t-> Ambient Light = {:?}\n\t-> Skybox = {:?}",
        map.0.name, map.0.name, map.0.path, map.0.meta.ambient_light, map.0.meta.sky
    );
    match &map.0.meta.map {
        MapData::Gltf { path } => {
            let mut map_path = map.0.path.clone();
            map_path.push(path);
            let asset_path = format!("{}#Scene0", map_path.to_str().unwrap());
            let scene = asset_server.load(asset_path.as_str());
            let instance_id = scene_spawner.spawn(scene.clone());
            commands.insert_resource(ArenaMapData {
                name: map.0.name.clone(),
                scene,
                instance_id,
                loaded: false,
            });
        }
    }
    commands.insert_resource(AmbientLight {
        color: crate::zombrr_color_to_bevy(&map.0.meta.ambient_light.color),
        brightness: map.0.meta.ambient_light.brightness,
    });
    commands
    .spawn_bundle(SkyBundle {
        sun: preset_to_sun(&map.0.meta.sky),
        mesh: meshes.add(Mesh::from(shape::Cube {
            size: map.0.meta.sky.sky_size,
        })),
        material: sky_materials.add(preset_to_material(&map.0.meta.sky.preset)),
        ..Default::default()
    })
    .insert(Name::new("Arena Map SkyBox"))
    .insert(crate::ArenaMapSkyBox);
}

fn preset_to_sun(sky: &Sky) -> Sun {
    Sun {
        latitude: sky.latitude as f64,
        longitude: sky.longitude as f64,
        simulation_seconds_per_second: 24.0 * 60.0 * 60.0
        / sky.day_length as f64,
        active: sky.active,
        distance: sky.distance,
        now: Utc.ymd(2021, 03, 01).and_hms(7, 0, 0),
    }
}

fn preset_to_material(preset: &SkyPreset) -> SkyMaterial {
    match preset {
        SkyPreset::BloodSky => SkyMaterial::blood_sky(),
        SkyPreset::AlientDay => SkyMaterial::alien_day(),
        SkyPreset::StellarDawn => SkyMaterial::stellar_dawn(),
        SkyPreset::RedSunset => SkyMaterial::red_sunset(),
        SkyPreset::BlueDusk => SkyMaterial::blue_dusk(),
        SkyPreset::PurpleDusk => SkyMaterial::purple_dusk(),
        SkyPreset::Custom {
            mie_k_coefficient,
            primaries,
            sun_position,
            depolarization_factor,
            luminance,
            mie_coefficient,
            mie_directional_g,
            mie_v,
            mie_zenith_length,
            num_molecules,
            rayleigh,
            rayleigh_zenith_length,
            refractive_index,
            sun_angular_diameter_degrees,
            sun_intensity_factor,
            sun_intensity_falloff_steepness,
            tonemap_weighting,
            turbidity,
        } => SkyMaterial {
            mie_k_coefficient: Vec4::from(*mie_k_coefficient),
            primaries: Vec4::from(*primaries),
            sun_position: Vec4::from(*sun_position),
            depolarization_factor: *depolarization_factor,
            luminance: *luminance,
            mie_coefficient: *mie_coefficient,
            mie_directional_g: *mie_directional_g,
            mie_v: *mie_v,
            mie_zenith_length: *mie_zenith_length,
            num_molecules: *num_molecules,
            rayleigh: *rayleigh,
            rayleigh_zenith_length: *rayleigh_zenith_length,
            refractive_index: *refractive_index,
            sun_angular_diameter_degrees: *sun_angular_diameter_degrees,
            sun_intensity_factor: *sun_intensity_factor,
            sun_intensity_falloff_steepness: *sun_intensity_falloff_steepness,
            tonemap_weighting: *tonemap_weighting,
            turbidity: *turbidity,
        },
    }
}
