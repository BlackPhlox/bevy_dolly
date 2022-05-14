use bevy::prelude::{Component, Deref, DerefMut};
use dolly::prelude::{LeftHanded, CameraRig};

#[derive(Component, Deref, DerefMut)]
pub struct CR(CameraRig<LeftHanded>);