use bevy::{
    app::PluginGroupBuilder,
    prelude::{App, Plugin, PluginGroup},
};

pub use dolly;

pub mod dolly_type;
pub mod map;
#[cfg(feature = "drivers")]
pub mod drivers;

//Todo: pub mod drivers
//Do it behind a default feature flag

pub mod prelude {
    pub use crate::{
        dolly::prelude::*, map::*, dolly_type::*, drivers::*, Dolly,
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
        group.add(Dolly);//.add(DollyPosCtrl);
    }
}
