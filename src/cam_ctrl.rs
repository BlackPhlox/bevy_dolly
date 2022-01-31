use bevy::prelude::{Plugin, App};

pub struct DollyCamCtrl;
impl Plugin for DollyCamCtrl {
    fn build(&self, app: &mut App) {
        app.init_resource::<DollyCamCtrlConfig>();
    }
}

struct DollyCamCtrlConfig {
    enabled: bool,
}

impl Default for DollyCamCtrlConfig {
    fn default() -> Self {
        DollyCamCtrlConfig {
            enabled: true,
        }
    }
}