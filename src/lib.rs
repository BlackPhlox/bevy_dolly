use bevy::{
    app::PluginGroupBuilder,
    input::Input,
    prelude::{AppBuilder, KeyCode, Mut, Plugin, PluginGroup, Res, Transform},
};
use cam_ctrl::DollyCamCtrl;
use ctrl::DollyDefaultCtrl;
use dolly::glam::{Mat3, Quat, Vec3};

pub mod cam_ctrl;
mod cone;
pub mod ctrl;
pub mod cursor_grab;
pub mod system;
pub mod system2;

pub struct Dolly;
impl Plugin for Dolly {
    fn build(&self, _app: &mut AppBuilder) {}
}

pub struct DollyPlugins;
impl PluginGroup for DollyPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(Dolly).add(DollyCamCtrl).add(DollyDefaultCtrl);
    }
}

pub trait Transform2Bevy {
    fn transform_2_bevy(&mut self, transform: dolly::transform::Transform);
}

impl Transform2Bevy for Mut<'_, Transform> {
    fn transform_2_bevy(&mut self, transform: dolly::transform::Transform) {
        let (translation, rotation) = transform.into_position_rotation();
        self.translation = bevy::math::Vec3::new(translation.x, translation.y, translation.z);
        self.rotation = bevy::math::Quat::from_xyzw(rotation.x, rotation.y, rotation.z, rotation.w);
    }
}

pub trait Transform2DollyMut {
    fn transform_2_dolly_mut(&self) -> dolly::transform::Transform;
}

impl Transform2DollyMut for Mut<'_, Transform> {
    fn transform_2_dolly_mut(&self) -> dolly::transform::Transform {
        let t = self.translation;
        let r = self.rotation;
        dolly::transform::Transform {
            position: Vec3::new(t.x, t.y, t.z),
            rotation: Quat::from_xyzw(r.x, r.y, r.z, r.w),
        }
    }
}

pub trait Transform2Dolly {
    fn transform_2_dolly(&self) -> dolly::transform::Transform;
}

impl Transform2Dolly for Transform {
    fn transform_2_dolly(&self) -> dolly::transform::Transform {
        let t = self.translation;
        let r = self.rotation;
        dolly::transform::Transform {
            position: Vec3::new(t.x, t.y, t.z),
            rotation: Quat::from_xyzw(r.x, r.y, r.z, r.w),
        }
    }
}

pub trait ZeroYRotation {
    fn zero_y_rotation(&self) -> dolly::glam::Quat;
}

impl ZeroYRotation for dolly::glam::Quat {
    fn zero_y_rotation(&self) -> dolly::glam::Quat {
        let mut forward = *self * Vec3::Z;
        let up = Vec3::Y;
        let right = up.cross(forward);
        forward = right.cross(up);
        let mat = Mat3::from_cols(right, up, forward);
        Quat::from_mat3(&mat)
    }
}

pub fn validate_key<T>(codes: &'static [T], key: &T) -> bool
where
    T: PartialEq<T>,
{
    codes.iter().any(|m| m == key)
}

trait AnyPressed {
    fn any_pressed(&self, codes: &'static [KeyCode]) -> bool;
}

impl AnyPressed for Res<'_, Input<KeyCode>> {
    fn any_pressed(&self, codes: &'static [KeyCode]) -> bool {
        for key in codes {
            if self.just_pressed(*key) {
                return true;
            }
        }
        false
    }
}
