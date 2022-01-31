use bevy::{
    app::PluginGroupBuilder,
    prelude::{App, Plugin, PluginGroup},
};
use pos_ctrl::DollyPosCtrl;

mod cone;
pub mod grab_cursor;
pub mod pos_ctrl;
pub mod transform_mapping;

pub use crate::grab_cursor::*;
pub use crate::pos_ctrl::*;
pub use crate::transform_mapping::*;

pub mod prelude {
    pub use crate::{rig::*, *};
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
