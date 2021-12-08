use bevy::prelude::*;
use bevy::asset::AssetServerSettings;
use bevy_devtools::DevToolsExt;

mod skip_menu;
mod change_map;
mod change_mode;
pub mod events;

fn main() {
    App::build()
        // Add resources
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AssetServerSettings {
            asset_folder: "../../assets".into()
        })
        .insert_resource(bevy::log::LogSettings {
            filter: "wgpu=warn".into(),
            level: bevy::log::Level::DEBUG
        })
        // Add plugins
        .add_plugins(zombrr::ZombrrPlugins)
        .insert_resource(zombrr::arena::modes::Mode::OneEnemy)

        // Development Tools
        .add_event::<events::ChangeMap>()
        .add_system(change_map::handle_change_map.system())
        .devtools_tool(change_map::change_map())
        .add_event::<events::ChangeMode>()
        .add_system(change_mode::handle_change_mode.system())
        .devtools_tool(change_mode::change_mode())
        // Temperary Systems
        .add_system_set(skip_menu::system_set())

        .run()
}
