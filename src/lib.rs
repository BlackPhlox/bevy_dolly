pub use dolly;
use dolly::handedness::RightHanded;

pub mod dolly_type;
#[cfg(feature = "drivers")]
pub mod drivers;
#[cfg(feature = "helpers")]
pub mod helpers;
pub mod system;

pub mod prelude {
    pub use crate::{
        dolly::{driver::*, drivers::*, handedness, util},
        dolly_type::*,
        drivers::{follow::*, fpv::*},
        helpers::*,
        helpers::{cone::*, cursor_grab::*, pos_ctrl::*},
        system::*,
    };
}

trait TransformConversion {
	fn into_bevy_transform(self) -> bevy::prelude::Transform;
}

impl TransformConversion for dolly::transform::Transform<RightHanded> {
	fn into_bevy_transform(self) -> bevy::prelude::Transform {
		bevy::prelude::Transform {
			translation: self.position,
			rotation: self.rotation,
			..Default::default()
		}
	}
}