use bevy::prelude::*;
use bevy_dolly::*;

#[test]
fn transform_2_bevy() {
    let mut t: Transform = Transform::from_xyz(1.0, 1.0, 1.0);
    t.transform_2_bevy(dolly::transform::Transform::IDENTITY);
    
    assert_eq!(t, Transform::from_xyz(0.0, 0.0, 0.0));
}