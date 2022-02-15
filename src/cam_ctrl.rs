use bevy::prelude::{App, Plugin};

pub struct DollyCamCtrl;
impl Plugin for DollyCamCtrl {
    fn build(&self, app: &mut App) {
        app.init_resource::<DollyCamCtrlConfig>();
    }
}

#[allow(dead_code)]
struct DollyCamCtrlConfig {
    enabled: bool,
}

impl Default for DollyCamCtrlConfig {
    fn default() -> Self {
        DollyCamCtrlConfig { enabled: true }
    }
}
