use bevy::{
    app::PluginGroupBuilder,
    input::Input,
    prelude::{AppBuilder, KeyCode, Mut, Plugin, PluginGroup, Res, Transform},
};
use cam_ctrl::DollyCamCtrl;
use ctrl::DollyDefaultCtrl;
use dolly::{
    glam::{EulerRot, Quat, Vec3},
    prelude::YawPitch,
};

pub mod cam_ctrl;
mod cone;
pub mod ctrl;
pub mod cursor_grab;
pub mod drivers;
pub mod plugins;

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

pub trait UpdateYawPitch {
    fn update_rotation(&mut self, yp: &YawPitch);
}

impl UpdateYawPitch for dolly::drivers::Rotation {
    fn update_rotation(&mut self, yp: &YawPitch) {
        self.rotation = dolly::glam::Quat::from_euler(
            EulerRot::YXZ,
            yp.yaw_degrees.to_radians(),
            yp.pitch_degrees.to_radians(),
            0.0,
        )
    }
}

pub trait ZeroedYRotation {
    fn zeroed_y_rotation(&self) -> dolly::glam::Quat;
}

impl ZeroedYRotation for dolly::glam::Quat {
    fn zeroed_y_rotation(&self) -> dolly::glam::Quat {
        let (mut euler, a) = self.to_axis_angle();
        euler.x = 0.;
        euler.z = 0.;
        dolly::glam::Quat::from_axis_angle(euler, a)
        /*
        Don't work as intended
        let mut forward = *self * Vec3::Z;
        let up = Vec3::Y;
        let right = up.cross(forward);
        forward = right.cross(up);
        let mat = Mat3::from_cols(right, up, forward);
        Quat::from_mat3(&mat)*/
    }
}

pub fn validate_key<T>(codes: &'static [T], key: &T) -> bool
where
    T: PartialEq<T>,
{
    codes.iter().any(|m| m == key)
}

pub trait IterAnyPressed {
    fn is_being_pressed(&self, key: &KeyCode) -> bool;
}

impl IterAnyPressed for &'static [KeyCode] {
    fn is_being_pressed(&self, key: &KeyCode) -> bool {
        self.iter().any(|m| m == key)
    }
}

pub trait AnyPressed {
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
