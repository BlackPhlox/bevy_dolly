use bevy::prelude::{AppBuilder, Plugin};
use ctrl::Ctrl;

pub mod cone;
pub mod ctrl;

pub struct Dolly;
impl Plugin for Dolly {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(Ctrl);
    }
}