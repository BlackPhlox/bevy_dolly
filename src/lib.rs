use bevy::{
    app::PluginGroupBuilder,
    prelude::{App, Plugin, PluginGroup},
};

pub use dolly;
use pos_ctrl::DollyPosCtrl;

mod cone;

pub mod grab_cursor;
pub mod pos_ctrl;
pub mod transform_mapping;

pub mod prelude {
    pub use crate::{
        dolly::prelude::*, grab_cursor::*, pos_ctrl::*, transform_mapping::*, Dolly, DollyPlugins,
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
