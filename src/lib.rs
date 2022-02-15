use bevy::{
    app::PluginGroupBuilder,
    prelude::{App, Plugin, PluginGroup},
};

pub use dolly;
use pos_ctrl::DollyPosCtrl;

mod cone;

pub mod cam_ctrl;
pub mod cursor_grab;
pub mod pos_ctrl;
pub mod transform_mapping;

pub mod prelude {
    pub use crate::{
        cam_ctrl::*, cursor_grab::*, dolly::prelude::*, pos_ctrl::*, transform_mapping::*, Dolly,
        DollyPlugins,
    };
}

pub struct Dolly;
impl Plugin for Dolly {
    fn build(&self, _app: &mut App) {}
}

pub struct DollyPlugins;
impl PluginGroup for DollyPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(Dolly).add(DollyPosCtrl);
    }
}
