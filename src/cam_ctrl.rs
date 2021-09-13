use bevy::prelude::*;

pub use crate::cursor_grab::DollyCursorGrab;

pub struct DollyCamCtrl;
impl Plugin for DollyCamCtrl {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<DollyCamCtrlConfig>()
            .add_plugin(DollyCursorGrab);
    }
}

#[derive(Default)]
pub struct DollyCamCtrlConfig {}

/*
impl Default for DollyCamCtrlConfig {
    fn default() -> Self {
        DollyCamCtrlConfig {}
    }
}*/
