use std::fmt::Debug;

use bevy::{
    app::PluginGroupBuilder,
    input::Input,
    prelude::{AppBuilder, KeyCode, Mut, Plugin, PluginGroup, Res, Transform},
};
use cam_ctrl::DollyCamCtrl;
use ctrl::DollyDefaultCtrl;
use dolly::{
    driver::RigDriver,
    glam::{EulerRot, Quat, Vec3},
    prelude::{CameraRig, YawPitch},
    rig::{CameraRigBuilder, RigUpdateParams},
};

pub mod cam_ctrl;
mod cone;
pub mod ctrl;
pub mod cursor_grab;
pub mod drivers;

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

pub trait ZeroYRotation {
    fn zero_y_rotation(&self) -> dolly::glam::Quat;
}

impl ZeroYRotation for dolly::glam::Quat {
    fn zero_y_rotation(&self) -> dolly::glam::Quat {
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

#[derive(Debug)]
pub struct SubRig<T> {
    pub driver: T,
    pub rig: CameraRig,
}

impl<T> RigDriver for SubRig<T>
where
    T: 'static + Debug,
{
    fn update(&mut self, params: RigUpdateParams) -> dolly::transform::Transform {
        let t = self.rig.update(params.delta_time_seconds);
        dolly::transform::Transform {
            position: t.position,
            rotation: t.rotation,
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
impl<T: WithRigSettings<S>, S> WithRigSettings<S> for SubRig<T> {
    fn init(settings: S) -> Self {
        SubRig {
            driver: T::init(settings),
            rig: CameraRig::builder().build(),
        }
    }
}

pub trait WithRigSettings<S> {
    fn init(settings: S) -> Self;
}

//V1
pub trait CustomBuild {
    fn with_rig<T, S>(self, settings: S) -> CameraRigBuilder
    where
        T: RigDriver + WithRigSettings<S> + Sync + Send;
}
impl CustomBuild for CameraRigBuilder {
    fn with_rig<T, S>(self, settings: S) -> CameraRigBuilder
    where
        T: RigDriver + WithRigSettings<S> + Sync + Send,
    {
        self.with(T::init(settings))
    }
}
//V2
pub trait SubRigBuild<S> {
    fn with_sub_rig<T>(self, settings: S) -> CameraRigBuilder
    where
        T: 'static + Debug + WithRigSettings<S> + Sync + Send;
}

impl<S> SubRigBuild<S> for CameraRigBuilder {
    fn with_sub_rig<T>(self, settings: S) -> CameraRigBuilder
    where
        T: 'static + Debug + WithRigSettings<S> + Sync + Send,
    {
        self.with(SubRig::<T>::init(settings))
    }
}
